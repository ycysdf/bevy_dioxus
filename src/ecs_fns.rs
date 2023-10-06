use std::any::TypeId;
use std::mem::ManuallyDrop;
use std::ops::DerefMut;
use std::ptr::NonNull;

use bevy::ecs::world::EntityMut;
use bevy::hierarchy::{BuildWorldChildren, Children, Parent};
use bevy::prelude::{
    AppTypeRegistry, Component, Entity, Reflect, ReflectFromReflect, Style, Text, warn, World,
};
use bevy::ptr::OwningPtr;
use bevy::reflect::{ReflectFromPtr, TypeRegistryInternal as TypeRegistry};
use bevy::text::TextLayoutInfo;

use crate::entity_extra_data::EntitiesExtraData;

pub fn dyn_reflect_to_owning_ptr(dyn_reflect: Box<dyn Reflect>) -> OwningPtr<'static> {
    let mut dyn_reflect = ManuallyDrop::new(dyn_reflect);
    let dyn_reflect = &mut **dyn_reflect;
    let ptr = dyn_reflect as *const dyn Reflect as *const ();

    let ptr = NonNull::<u8>::new(ptr as *mut u8).unwrap();
    unsafe { OwningPtr::new(ptr) }
}

fn clone_entity<'a>(world: &'a mut World, entities_extra_data: &'a mut EntitiesExtraData, entity: Entity) -> EntityMut<'a> {
    let component_infos = world.inspect_entity(entity);
    let mut components = vec![];
    let mut component_ids = vec![];

    let type_ids = vec![TypeId::of::<Parent>(), TypeId::of::<Children>()];

    let mut is_text = false;
    for component_info in component_infos.iter() {
        let component_type_id = component_info.type_id().unwrap();
        if type_ids.contains(&component_type_id) {
            continue;
        }
        if component_type_id == TypeId::of::<Text>() {
            is_text = true;
        }
        let component_id = component_info.id();
        let Some(reflect_obj) = ({
            let type_id = world
                .components()
                .get_info(component_id)
                .and_then(|n| n.type_id())
                .unwrap();
            let component_ptr = world.get_by_id(entity, component_id).unwrap();

            let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
            let type_registry = type_registry.read();
            match type_registry.get_type_data::<ReflectFromPtr>(type_id) {
                None => None,
                Some(from_ptr) => Some(unsafe { from_ptr.as_reflect_ptr(component_ptr) }),
            }
        }) else {
            if component_type_id != TypeId::of::<TextLayoutInfo>() {
                warn!("No Found TypeData: {:#?}", component_info.name());
            }
            continue;
        };
        let reflect_obj = reflect_obj
            .clone_real_value(
                &world.resource::<AppTypeRegistry>().read(),
                component_type_id,
            )
            .unwrap();
        component_ids.push(component_id);
        components.push(reflect_obj);
    }
    let entity_extra_data = entities_extra_data.get(&entity).cloned();
    let mut loaded_entity = world.spawn_empty();
    if let Some(entity_extra_data) = entity_extra_data {
        entities_extra_data.insert(loaded_entity.id(), entity_extra_data);
    }

    if is_text {
        loaded_entity.insert(TextLayoutInfo::default());
    }
    unsafe {
        loaded_entity.insert_by_ids(
            component_ids.as_slice(),
            components.into_iter().map(dyn_reflect_to_owning_ptr),
        )
    };
    loaded_entity
}

pub fn clone_entity_nest<'a>(world: &'a mut World, entities_extra_data: &'a mut EntitiesExtraData, entity: Entity) -> Entity {
    let children = world
        .get::<Children>(entity)
        .map(|n| n.iter().copied().collect::<Vec<_>>());

    let new_entity = clone_entity(world, entities_extra_data, entity).id();
    if let Some(children) = children {
        let mut new_children_entities = Vec::with_capacity(children.len());
        for child_entity in children {
            let child_entity = clone_entity_nest(world, entities_extra_data, child_entity);
            new_children_entities.push(child_entity);
        }
        world
            .entity_mut(new_entity)
            .push_children(&new_children_entities);
    }
    // loaded_entity
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
