#![allow(non_camel_case_types)]

use std::str::FromStr;

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::ui;
use bevy::ui::widget::UiImageSize;

use crate::dom_commands::DomAttributeValue;
use crate::ecs_fns::StyleEntityExt;
use crate::element_core::ElementAttr;

use crate::entity_extra_data::{EntitiesExtraData, EntityExtraData};
use crate::tailwind::handle_classes;
use crate::{
    get_element_type, set_text_value, ElementTypeUnTyped, ReflectTextStyledElementType,
    TextStyledElementType,
};

pub use attr_values::*;
pub use composite_attrs::*;
pub use attrs::*;
mod attr_values;
mod attrs;
pub mod composite_attrs;

pub trait MyFromStr: Sized {
    fn from_str(s: &str) -> Option<Self>;
}

pub fn from_str<T: MyFromStr>(s: &str) -> Option<T> {
    T::from_str(s)
}

pub struct SetAttrValueContext<'w, 'e> {
    pub entities_extra_data: &'e mut EntitiesExtraData,
    pub entity_ref: &'w mut EntityMut<'w>,
    pub type_registry: AppTypeRegistry,
}

impl<'w, 'e> SetAttrValueContext<'w, 'e> {
    pub fn entity_extra_data(&mut self) -> &mut EntityExtraData {
        self.entities_extra_data
            .get_mut(&self.entity_ref.id())
            .unwrap()
    }
    pub fn entity_mut_scope<U>(&mut self, entity: Entity, f: impl FnOnce(&mut EntityMut) -> U) {
        self.entity_ref
            .world_scope(|world| f(&mut world.entity_mut(entity)));
    }

    pub fn element_type(&mut self) -> &'static dyn ElementTypeUnTyped {
        get_element_type(self.entity_extra_data().schema_name)
    }

    pub fn get_text_element_type(&mut self) -> Option<&'static dyn TextStyledElementType> {
        self.get_entity_text_element_type(self.entity_ref.id())
    }

    pub fn get_entity_text_element_type(
        &mut self,
        entity: Entity,
    ) -> Option<&'static dyn TextStyledElementType> {
        let schema_name = self
            .entities_extra_data
            .get(&entity)
            .map(|n| n.schema_name)?;
        let schema_type = get_element_type(schema_name);
        let type_registry = self.type_registry.read();
        type_registry
            .get_type_data::<ReflectTextStyledElementType>(schema_type.type_id())
            .and_then(|n| n.get(schema_type.as_reflect()))
    }
}
