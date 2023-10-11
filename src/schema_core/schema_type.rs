use std::any::Any;

use bevy::ecs::component::ComponentInfo;
use bevy::ecs::world::EntityMut;
use bevy::prelude::World;
use bevy::reflect::ReflectFromPtr;

use crate::prelude::{AppTypeRegistry, Entity, warn};
use crate::ReflectExtension;
use crate::schema_core::SchemaPropUntyped;

pub trait SchemaTypeUnTyped {
    fn schema_name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;

    fn props(&self) -> &'static [&'static dyn SchemaPropUntyped];

    fn prop(&self, attr_name: &str) -> Option<&'static dyn SchemaPropUntyped>;
    fn prop_by_index(&self, index: u8) -> &'static dyn SchemaPropUntyped {
        self.props()[index as usize]
    }
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w>;
    fn try_insert_no_reflect_components(
        &self,
        entity_mut: &mut EntityMut,
        template_world: &World,
        template_entity: Entity,
        type_registry: AppTypeRegistry,
        component_info: &ComponentInfo,
    ) -> bool;
}

impl<T: SchemaTypeBase + SchemaType> SchemaTypeUnTyped for T {
    #[inline]
    fn schema_name(&self) -> &'static str {
        T::TAG_NAME
    }

    #[inline]
    fn namespace(&self) -> Option<&'static str> {
        T::NAME_SPACE
    }

    #[inline]
    fn props(&self) -> &'static [&'static dyn SchemaPropUntyped] {
        T::PROPS
    }

    #[inline]
    fn prop(&self, attr_name: &str) -> Option<&'static dyn SchemaPropUntyped> {
        T::prop(attr_name)
    }
    #[inline]
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        self.spawn(world)
    }

    #[inline]
    fn try_insert_no_reflect_components(
        &self,
        entity_mut: &mut EntityMut,
        template_world: &World,
        template_entity: Entity,
        type_registry: AppTypeRegistry,
        component_info: &ComponentInfo,
    ) -> bool {
        self.try_insert_no_reflect_components(
            entity_mut,
            template_world,
            template_entity,
            type_registry,
            component_info,
        )
    }
}

pub trait SchemaTypeBase {
    const TAG_NAME: &'static str;
    const NAME_SPACE: Option<&'static str> = None;
    const PROPS: &'static [&'static dyn SchemaPropUntyped];
    const NAME: &'static str = Self::TAG_NAME;

    fn prop(attr_name: &str) -> Option<&'static dyn SchemaPropUntyped> {
        Self::PROPS.iter().find(|n| n.name() == attr_name).copied()
    }
}

pub fn default_clone_component(
    world: &World,
    type_registry: AppTypeRegistry,
    entity: Entity,
    component_info: &ComponentInfo,
) -> Option<Box<dyn Any>> {
    let component_type_id = component_info.type_id()?;

    let component_id = component_info.id();

    let type_id = world
        .components()
        .get_info(component_id)
        .and_then(|n| n.type_id())?;
    let component_ptr = world.get_by_id(entity, component_id)?;

    let type_registry = type_registry.read();
    let from_ptr = type_registry.get_type_data::<ReflectFromPtr>(type_id);
    if from_ptr.is_none() {
        warn!(
            "component {:?} no found ReflectFromPtr type data!",
            component_info.name()
        );
    }
    let from_ptr = from_ptr?;
    let reflect_obj = unsafe { from_ptr.as_reflect_ptr(component_ptr) };

    let reflect_obj = reflect_obj.clone_real_value(&type_registry, component_type_id)?;
    Some(reflect_obj.into_any())
}

pub trait SchemaType {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w>;
    #[inline]
    fn try_insert_no_reflect_components(
        &self,
        _entity_mut: &mut EntityMut,
        _template_world: &World,
        _template_entity: Entity,
        _type_registry: AppTypeRegistry,
        _component_info: &ComponentInfo,
    ) -> bool {
        false
    }
}
