use std::any::Any;

use bevy::ecs::component::ComponentInfo;
use bevy::ecs::world::EntityMut;
use bevy::prelude::World;
use bevy::reflect::{Reflect, ReflectFromPtr};

use crate::element_core::ElementAttrUntyped;
use crate::prelude::{warn, AppTypeRegistry, Entity};
use crate::{ElementCompositeAttrUntyped, ReflectExtension};

pub trait ElementTypeUnTyped: Reflect {
    fn name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;

    fn attrs(&self) -> &'static [&'static dyn ElementAttrUntyped];

    fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementAttrUntyped>;
    fn composite_attrs(&self) -> &'static [&'static dyn ElementCompositeAttrUntyped];
    fn composite_attr(&self, attr_name: &str) -> Option<&'static dyn ElementCompositeAttrUntyped>;
    fn attr_by_index(&self, index: u8) -> &'static dyn ElementAttrUntyped {
        self.attrs()[index as usize]
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

impl<T: ElementTypeBase + ElementType> ElementTypeUnTyped for T {
    #[inline]
    fn name(&self) -> &'static str {
        T::TAG_NAME
    }

    #[inline]
    fn namespace(&self) -> Option<&'static str> {
        T::NAME_SPACE
    }

    #[inline]
    fn attrs(&self) -> &'static [&'static dyn ElementAttrUntyped] {
        T::ATTRS
    }

    #[inline]
    fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementAttrUntyped> {
        T::attr(attr_name)
    }

    #[inline]
    fn composite_attrs(&self) -> &'static [&'static dyn ElementCompositeAttrUntyped] {
        T::COMPOSITE_ATTRS
    }

    #[inline]
    fn composite_attr(&self, attr_name: &str) -> Option<&'static dyn ElementCompositeAttrUntyped> {
        T::composite_attr(attr_name)
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

pub trait ElementTypeBase: Reflect {
    const TAG_NAME: &'static str;
    const NAME_SPACE: Option<&'static str> = None;
    const ATTRS: &'static [&'static dyn ElementAttrUntyped];
    const COMPOSITE_ATTRS: &'static [&'static dyn ElementCompositeAttrUntyped];
    const NAME: &'static str = Self::TAG_NAME;

    fn attr(attr_name: &str) -> Option<&'static dyn ElementAttrUntyped> {
        Self::ATTRS.iter().find(|n| n.name() == attr_name).copied()
    }

    fn composite_attr(attr_name: &str) -> Option<&'static dyn ElementCompositeAttrUntyped> {
        Self::COMPOSITE_ATTRS.iter().find(|n| n.name() == attr_name).copied()
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

pub trait ElementType {
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
