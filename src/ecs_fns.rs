use std::any::{Any, TypeId};
use std::mem::ManuallyDrop;
use std::ops::DerefMut;
use std::ptr::NonNull;

use bevy::ecs::component::ComponentInfo;
use bevy::ecs::world::EntityMut;
use bevy::hierarchy::{BuildWorldChildren, Children, Parent};
use bevy::prelude::{
    AppTypeRegistry, Component, Entity, Reflect, ReflectFromReflect, Style, warn, World,
};
use bevy::ptr::OwningPtr;
use bevy::reflect::{ReflectFromPtr, TypeRegistryInternal as TypeRegistry};

use crate::entity_extra_data::EntitiesExtraData;
use crate::get_schema_type;
use crate::prelude::{
    default, Display, error, Name, NodeBundle, ReflectComponent,
};

pub fn dyn_to_owning_ptr(dyn_reflect: Box<dyn Any>) -> OwningPtr<'static> {
    let mut dyn_reflect = ManuallyDrop::new(dyn_reflect);
    let dyn_reflect = &mut **dyn_reflect;
    let ptr = dyn_reflect as *const dyn Any as *const ();

    let ptr = NonNull::<u8>::new(ptr as *mut u8).unwrap();
    unsafe { OwningPtr::new(ptr) }
}

pub fn empty_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::None,
            ..default()
        },
        ..default()
    }
}

fn clone_entity<'a>(
    world: &'a mut World,
    entities_extra_data: &'a mut EntitiesExtraData,
    template_world: &mut World,
    template_entities_extra_data: &mut EntitiesExtraData,
    template_entity: Entity,
) -> EntityMut<'a> {
    let template_world = &*template_world;
    if template_entities_extra_data
        .empty_node_entities
        .contains(&template_entity)
    {
        return world.spawn((
            empty_node(),
            template_world
                .get::<Name>(template_entity)
                .cloned()
                .unwrap_or(Name::new("[Empty Node]")),
        ));
    }

    let entity_extra_data = template_entities_extra_data.get(&template_entity).cloned();
    let Some(entity_extra_data) = entity_extra_data else {
        error!("No Found Entity Extra Data : {:?}", template_entity);
        return world.spawn((
            empty_node(),
            template_world
                .get::<Name>(template_entity)
                .cloned()
                .unwrap_or(Name::new("[Empty Node] [Error]")),
        ));
    };
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let schema_type = get_schema_type(entity_extra_data.schema_name);
    let ignore_type_ids = vec![TypeId::of::<Parent>(), TypeId::of::<Children>()];

    let mut components = vec![];
    // let mut uninited_components = vec![];
    let mut component_ids = vec![];
    let template_entity_ref = template_world.entity(template_entity);
    let component_infos = template_entity_ref
        .archetype()
        .components()
        .filter_map(|n| {
            let Some(component_info) = template_world.components().get_info(n) else {
                warn!("component_info no found by id {:?}!", n);
                return None;
            };
            let Some(component_type_id) = ComponentInfo::type_id(component_info) else {
                warn!("component {:#?} type_id is null!", component_info.name());
                return None;
            };
            if ignore_type_ids.contains(&component_type_id) {
                return None;
            }
            Some((component_info, world.components().get_id(component_type_id)))
        })
        .collect::<Vec<_>>();
    let mut loaded_entity = world.spawn_empty();

    for (component_info, component_id) in component_infos {
        if schema_type.try_insert_no_reflect_components(
            &mut loaded_entity,
            template_world,
            template_entity,
            type_registry.clone(),
            component_info,
        ) {
            continue;
        }
        let component_type_id = component_info.type_id().unwrap();
        let type_registry = type_registry.read();

        let reflect_obj = {
            let Some(component_ptr) = template_entity_ref.get_by_id(component_info.id()) else {
                warn!("component {:#?} no found!", component_info.name());
                continue;
            };

            let Some(from_ptr) = type_registry.get_type_data::<ReflectFromPtr>(component_type_id)
                else {
                    warn!(
                    "component {:#?} get ReflectFromPtr type data failed!",
                    component_info.name()
                );
                    continue;
                };
            unsafe { from_ptr.as_reflect_ptr(component_ptr) }
        };
        match component_id {
            None => {
                let Some(reflect_component) =
                    type_registry.get_type_data::<ReflectComponent>(component_type_id)
                    else {
                        warn!(
                        "component {:#?} no found ReflectComponent",
                        component_info.name()
                    );
                        continue;
                    };

                reflect_component.insert(&mut loaded_entity, reflect_obj)
            }
            Some(component_id) => {
                component_ids.push(component_id);
                components.push(
                    reflect_obj
                        .clone_real_value(&type_registry, component_type_id)
                        .unwrap()
                        .into_any(),
                );
            }
        }
    }

    unsafe {
        loaded_entity.insert_by_ids(
            component_ids.as_slice(),
            components.into_iter().map(dyn_to_owning_ptr),
        )
    };
    entities_extra_data.insert(loaded_entity.id(), entity_extra_data);
    loaded_entity
}

pub fn clone_entity_nest<'a>(
    world: &'a mut World,
    entities_extra_data: &'a mut EntitiesExtraData,
    template_world: &'a mut World,
    template_entities_extra_data: &'a mut EntitiesExtraData,
    template_entity: Entity,
) -> Entity {
    let children = template_world
        .get::<Children>(template_entity)
        .map(|n| n.iter().copied().collect::<Vec<_>>());

    let new_entity = clone_entity(
        world,
        entities_extra_data,
        template_world,
        template_entities_extra_data,
        template_entity,
    )
        .id();

    if let Some(children) = children {
        let mut new_children_entities = Vec::with_capacity(children.len());
        for child_entity in children {
            let child_entity = clone_entity_nest(
                world,
                entities_extra_data,
                template_world,
                template_entities_extra_data,
                child_entity,
            );
            new_children_entities.push(child_entity);
        }

        world
            .entity_mut(new_entity)
            .push_children(&new_children_entities);
    }
    new_entity
}

pub fn insert_before<'w, 'a>(
    world: &'w mut World,
    entity: Entity,
    inserted_entities: &'a [Entity],
) -> (EntityMut<'w>, usize) {
    let parent = world.get::<Parent>(entity).unwrap().get();
    let children = world.get::<Children>(parent).unwrap();
    let entity_index = children.iter().position(|n| *n == entity).unwrap();

    let mut parent_ref = world.entity_mut(parent);
    parent_ref.insert_children(entity_index, inserted_entities);
    (parent_ref, entity_index)
}

pub fn insert_after<'w, 'a>(
    world: &'w mut World,
    entity: Entity,
    inserted_entities: &'a [Entity],
) -> (EntityMut<'w>, usize) {
    let parent = world.get::<Parent>(entity).unwrap().get();
    let children = world.get::<Children>(parent).unwrap();
    let entity_index = children.iter().position(|n| *n == entity).unwrap();

    let mut parent_ref = world.entity_mut(parent);
    parent_ref.insert_children(entity_index + 1, inserted_entities);
    (parent_ref, entity_index)
}

pub trait ReflectExtension {
    fn clone_real_value(
        &self,
        type_registry: &TypeRegistry,
        type_id: TypeId,
    ) -> Option<Box<dyn Reflect>>;
}

impl ReflectExtension for dyn Reflect {
    fn clone_real_value(
        &self,
        type_registry: &TypeRegistry,
        type_id: TypeId,
    ) -> Option<Box<dyn Reflect>> {
        let from_reflect = type_registry.get_type_data::<ReflectFromReflect>(type_id)?;
        from_reflect.from_reflect(self)
    }
}

pub trait StyleEntityExt {
    fn try_set_style(&mut self, set_f: impl FnOnce(&mut Style));
    fn try_set<T: Component>(&mut self, set_f: impl FnOnce(&mut T));
}

impl StyleEntityExt for EntityMut<'_> {
    #[inline]
    fn try_set_style(&mut self, set_f: impl FnOnce(&mut Style)) {
        if let Some(mut style) = self.get_mut::<Style>() {
            set_f(style.deref_mut());
        }
    }
    #[inline]
    fn try_set<T: Component>(&mut self, set_f: impl FnOnce(&mut T)) {
        if let Some(mut component) = self.get_mut::<T>() {
            set_f(component.deref_mut());
        }
    }
}

pub trait WorldExtension {
    fn get_child_by_index(&mut self, entity: Entity, child_index: usize) -> Entity;
}

impl WorldExtension for World {
    fn get_child_by_index(&mut self, entity: Entity, child_index: usize) -> Entity {
        let children = self.entity(entity).get::<Children>().unwrap();
        children[child_index]
    }
}
