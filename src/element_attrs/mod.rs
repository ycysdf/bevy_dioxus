#![allow(non_camel_case_types)]

use std::str::FromStr;

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;

pub use attr_values::*;
pub use attrs::*;
pub use composite_attrs::*;

use crate::{attrs_trait_define, composite_attrs_trait_define, ElementCompositeAttr, ElementTypeUnTyped, get_element_type};
use crate::ecs_fns::StyleEntityExt;
use crate::element_core::ElementAttr;
use crate::entity_extra_data::{EntitiesExtraData, EntityExtraData};

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
}


attrs_trait_define!(CommonAttrs;0;
    class,
    name,
    z_index,
    background,
    border_left,
    border_right,
    border_top,
    border_bottom,
    border_color,
    display,
    position_type,
    overflow_x,
    overflow_y,
    direction,
    left,
    right,
    top,
    bottom,
    width,
    height,
    min_width,
    min_height,
    max_width,
    max_height,
    margin_left,
    margin_right,
    margin_top,
    margin_bottom,
    padding_left,
    padding_right,
    padding_top,
    padding_bottom,
    aspect_ratio,
    align_items,
    justify_items,
    align_self,
    justify_self,
    align_content,
    justify_content,
    flex_direction,
    flex_wrap,
    flex_grow,
    flex_shrink,
    flex_basis,
    column_gap,
    row_gap,
    visibility,
    transation,
    rotation,
    scale,
    text_color,
    font_size,
    text_linebreak,
    text_align,
    font
);

composite_attrs_trait_define!(CommonCompositeAttrs;
    margin,
    padding,
    border,
    transform
);