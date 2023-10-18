use std::any::Any;

use bevy::ecs::component::ComponentInfo;
use bevy::ecs::world::EntityMut;
use bevy::prelude::World;
use bevy::reflect::{Reflect, ReflectFromPtr};

use crate::{ElementCompositeAttrUntyped, ReflectExtension};
use crate::element_core::ElementAttrUntyped;
use crate::entity_extra_data::AttrIndex;
use crate::prelude::{AppTypeRegistry, Entity, warn};

pub trait ElementTypeUnTyped: Reflect {
    fn tag_name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;

    fn attrs(&self) -> &'static [&'static [&'static dyn ElementAttrUntyped]];

    fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementAttrUntyped>;
    fn composite_attrs(&self) -> &'static [&'static [&'static dyn ElementCompositeAttrUntyped]];
    fn composite_attr(&self, attr_name: &str) -> Option<&'static dyn ElementCompositeAttrUntyped>;
    fn attr_by_index(&self, index: AttrIndex) -> &'static dyn ElementAttrUntyped {
        let mut index = index as usize;
        for attrs in self.attrs() {
            if index < attrs.len() {
                return attrs[index];
            }
            index -= attrs.len();
        }
        unreachable!();
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
    fn tag_name(&self) -> &'static str {
        T::TAG_NAME
    }

    #[inline]
    fn namespace(&self) -> Option<&'static str> {
        T::NAME_SPACE
    }

    #[inline]
    fn attrs(&self) -> &'static [&'static [&'static dyn ElementAttrUntyped]] {
        T::ATTRS
    }

    #[inline]
    fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementAttrUntyped> {
        T::attr(attr_name)
    }

    #[inline]
    fn composite_attrs(&self) -> &'static [&'static [&'static dyn ElementCompositeAttrUntyped]] {
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
    const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped]];
    const COMPOSITE_ATTRS: &'static [&'static [&'static dyn ElementCompositeAttrUntyped]];

    fn attr(attr_name: &str) -> Option<&'static dyn ElementAttrUntyped> {
        use bevy::utils::HashMap;
        static ATTRS: std::sync::OnceLock<
            HashMap<&'static str, &'static dyn crate::ElementAttrUntyped>,
        > = std::sync::OnceLock::new();
        let map = ATTRS.get_or_init(|| {
            let mut map: HashMap<&'static str, &'static dyn crate::ElementAttrUntyped> =
                HashMap::new();
            for attrs in Self::ATTRS {
                for attr in *attrs {
                    map.insert(attr.attr_name(), *attr);
                }
            }
            map
        });
        map.get(attr_name).map(|n| *n)
    }

    fn composite_attr(attr_name: &str) -> Option<&'static dyn ElementCompositeAttrUntyped> {
        use bevy::utils::HashMap;
        static ATTRS: std::sync::OnceLock<
            HashMap<&'static str, &'static dyn crate::ElementCompositeAttrUntyped>,
        > = std::sync::OnceLock::new();
        let map = ATTRS.get_or_init(|| {
            let mut map: HashMap<&'static str, &'static dyn crate::ElementCompositeAttrUntyped> =
                HashMap::new();
            for attrs in Self::COMPOSITE_ATTRS {
                for attr in *attrs {
                    map.insert(attr.name(), *attr);
                }
            }
            map
        });
        map.get(attr_name).map(|n| *n)
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
