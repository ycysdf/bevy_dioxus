use bevy::{
    ecs::world::EntityMut,
    prelude::{Children, Entity},
    reflect::reflect_trait,
};

pub use element_input::*;
pub use element_text::*;
pub use element_view::*;

use crate::{element_core::ElementTypeUnTyped, ElementAttr, SetAttrValueContext};
use crate::element_attrs;

mod element_input;
mod element_text;
mod element_view;

#[reflect_trait]
pub trait TextStyledElementType {
    fn set_font(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::font as ElementAttr>::Value,
    );
    fn set_font_size(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::font_size as ElementAttr>::Value,
    );
    fn set_text_color(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::text_color as ElementAttr>::Value,
    );
    fn set_text_linebreak(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::text_linebreak as ElementAttr>::Value,
    );
    fn set_text_align(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::text_align as ElementAttr>::Value,
    );
}

pub fn context_children_scope(
    context: &mut SetAttrValueContext,
    mut f: impl FnMut(Entity, &mut SetAttrValueContext),
) {
    let Some(children) = context.entity_ref.get_mut::<Children>() else {
        return;
    };
    let children: Vec<Entity> = children.into_iter().copied().collect();
    for entity in children {
        f(entity, context);
    }
}

pub fn set_text_value(
    context: &mut SetAttrValueContext,
    mut f: impl FnMut(&'static dyn TextStyledElementType, &mut EntityMut),
) {
    if let Some(text_element_type) = context.get_text_element_type() {
        f(text_element_type, context.entity_ref);
    } else {
        context_children_scope(context, move |entity, context| {
            let Some(text_element_type) = context.get_entity_text_element_type(entity) else {
                return;
            };
            context.entity_mut_scope(entity, |entity_ref| {
                f(text_element_type, entity_ref);
            });
        });
    }
}

pub fn try_get_element_type(name: &str) -> Option<&'static dyn ElementTypeUnTyped> {
    match name {
        stringify!(view) => Some(&view),
        stringify!(text) => Some(&text),
        stringify!(input) => Some(&input),
        // stringify!(svg) => Some(&svg),
        _ => None,
    }
}

pub fn get_element_type(name: &str) -> &'static dyn ElementTypeUnTyped {
    try_get_element_type(name).expect(&format!("No Found ElementType by {:#?}", name))
}

#[macro_export]
macro_rules! common_attrs_define {
    () => {
        pub const class: crate::DioxusAttributeDescription =
            crate::element_attrs::class::ATTRIBUTE_DESCRIPTION;
        pub const name: crate::DioxusAttributeDescription =
            crate::element_attrs::name::ATTRIBUTE_DESCRIPTION;
        pub const z_index: crate::DioxusAttributeDescription =
            crate::element_attrs::z_index::ATTRIBUTE_DESCRIPTION;
        pub const background: crate::DioxusAttributeDescription =
            crate::element_attrs::background::ATTRIBUTE_DESCRIPTION;
        pub const border: crate::DioxusAttributeDescription =
            crate::element_attrs::border::ATTRIBUTE_DESCRIPTION;
        pub const border_color: crate::DioxusAttributeDescription =
            crate::element_attrs::border_color::ATTRIBUTE_DESCRIPTION;
        pub const display: crate::DioxusAttributeDescription =
            crate::element_attrs::display::ATTRIBUTE_DESCRIPTION;
        pub const position_type: crate::DioxusAttributeDescription =
            crate::element_attrs::position_type::ATTRIBUTE_DESCRIPTION;
        pub const overflow: crate::DioxusAttributeDescription =
            crate::element_attrs::overflow::ATTRIBUTE_DESCRIPTION;
        pub const direction: crate::DioxusAttributeDescription =
            crate::element_attrs::direction::ATTRIBUTE_DESCRIPTION;
        pub const left: crate::DioxusAttributeDescription =
            crate::element_attrs::left::ATTRIBUTE_DESCRIPTION;
        pub const right: crate::DioxusAttributeDescription =
            crate::element_attrs::right::ATTRIBUTE_DESCRIPTION;
        pub const top: crate::DioxusAttributeDescription =
            crate::element_attrs::top::ATTRIBUTE_DESCRIPTION;
        pub const bottom: crate::DioxusAttributeDescription =
            crate::element_attrs::bottom::ATTRIBUTE_DESCRIPTION;
        pub const width: crate::DioxusAttributeDescription =
            crate::element_attrs::width::ATTRIBUTE_DESCRIPTION;
        pub const height: crate::DioxusAttributeDescription =
            crate::element_attrs::height::ATTRIBUTE_DESCRIPTION;
        pub const min_width: crate::DioxusAttributeDescription =
            crate::element_attrs::min_width::ATTRIBUTE_DESCRIPTION;
        pub const min_height: crate::DioxusAttributeDescription =
            crate::element_attrs::min_height::ATTRIBUTE_DESCRIPTION;
        pub const max_width: crate::DioxusAttributeDescription =
            crate::element_attrs::max_width::ATTRIBUTE_DESCRIPTION;
        pub const max_height: crate::DioxusAttributeDescription =
            crate::element_attrs::max_height::ATTRIBUTE_DESCRIPTION;
        pub const margin: crate::DioxusAttributeDescription =
            crate::element_attrs::margin::ATTRIBUTE_DESCRIPTION;
        pub const padding: crate::DioxusAttributeDescription =
            crate::element_attrs::padding::ATTRIBUTE_DESCRIPTION;
        pub const aspect_ratio: crate::DioxusAttributeDescription =
            crate::element_attrs::aspect_ratio::ATTRIBUTE_DESCRIPTION;
        pub const align_items: crate::DioxusAttributeDescription =
            crate::element_attrs::align_items::ATTRIBUTE_DESCRIPTION;
        pub const justify_items: crate::DioxusAttributeDescription =
            crate::element_attrs::justify_items::ATTRIBUTE_DESCRIPTION;
        pub const align_self: crate::DioxusAttributeDescription =
            crate::element_attrs::align_self::ATTRIBUTE_DESCRIPTION;
        pub const justify_self: crate::DioxusAttributeDescription =
            crate::element_attrs::justify_self::ATTRIBUTE_DESCRIPTION;
        pub const align_content: crate::DioxusAttributeDescription =
            crate::element_attrs::align_content::ATTRIBUTE_DESCRIPTION;
        pub const justify_content: crate::DioxusAttributeDescription =
            crate::element_attrs::justify_content::ATTRIBUTE_DESCRIPTION;
        pub const flex_direction: crate::DioxusAttributeDescription =
            crate::element_attrs::flex_direction::ATTRIBUTE_DESCRIPTION;
        pub const flex_wrap: crate::DioxusAttributeDescription =
            crate::element_attrs::flex_wrap::ATTRIBUTE_DESCRIPTION;
        pub const flex_grow: crate::DioxusAttributeDescription =
            crate::element_attrs::flex_grow::ATTRIBUTE_DESCRIPTION;
        pub const flex_shrink: crate::DioxusAttributeDescription =
            crate::element_attrs::flex_shrink::ATTRIBUTE_DESCRIPTION;
        pub const flex_basis: crate::DioxusAttributeDescription =
            crate::element_attrs::flex_basis::ATTRIBUTE_DESCRIPTION;
        pub const visibility: crate::DioxusAttributeDescription =
            crate::element_attrs::visibility::ATTRIBUTE_DESCRIPTION;
        pub const transform: crate::DioxusAttributeDescription =
            crate::element_attrs::transform::ATTRIBUTE_DESCRIPTION;
        pub const transation: crate::DioxusAttributeDescription =
            crate::element_attrs::transation::ATTRIBUTE_DESCRIPTION;
        pub const rotation: crate::DioxusAttributeDescription =
            crate::element_attrs::rotation::ATTRIBUTE_DESCRIPTION;
        pub const scale: crate::DioxusAttributeDescription =
            crate::element_attrs::scale::ATTRIBUTE_DESCRIPTION;
        pub const text_color: crate::DioxusAttributeDescription =
            crate::element_attrs::text_color::ATTRIBUTE_DESCRIPTION;
        pub const font_size: crate::DioxusAttributeDescription =
            crate::element_attrs::font_size::ATTRIBUTE_DESCRIPTION;
        pub const text_linebreak: crate::DioxusAttributeDescription =
            crate::element_attrs::text_linebreak::ATTRIBUTE_DESCRIPTION;
        pub const text_align: crate::DioxusAttributeDescription =
            crate::element_attrs::text_align::ATTRIBUTE_DESCRIPTION;
        pub const font: crate::DioxusAttributeDescription =
            crate::element_attrs::font::ATTRIBUTE_DESCRIPTION;
    };
}

#[macro_export]
macro_rules! impl_element_type_base {
    ($(#[$attr:meta])*$name:ident,$($prop:ident),*) => {
        use crate::element_core::ElementAttr;
        $( #[$attr] )*
        pub struct $name;
        impl $name {
            crate::common_attrs_define!();
            $(
                pub const $prop: crate::DioxusAttributeDescription = $prop::ATTRIBUTE_DESCRIPTION;
            )*
        }
        impl crate::ElementTypeBase for $name {
            const TAG_NAME: &'static str = stringify!($name);
            const PROPS: &'static [&'static dyn crate::ElementAttrUntyped] = &[
                &crate::element_attrs::class,
                &crate::element_attrs::name,
                &crate::element_attrs::z_index,
                &crate::element_attrs::background,
                &crate::element_attrs::border,
                &crate::element_attrs::border_color,
                &crate::element_attrs::display,
                &crate::element_attrs::position_type,
                &crate::element_attrs::overflow,
                &crate::element_attrs::direction,
                &crate::element_attrs::left,
                &crate::element_attrs::right,
                &crate::element_attrs::top,
                &crate::element_attrs::bottom,
                &crate::element_attrs::width,
                &crate::element_attrs::height,
                &crate::element_attrs::min_width,
                &crate::element_attrs::min_height,
                &crate::element_attrs::max_width,
                &crate::element_attrs::max_height,
                &crate::element_attrs::margin,
                &crate::element_attrs::padding,
                &crate::element_attrs::aspect_ratio,
                &crate::element_attrs::align_items,
                &crate::element_attrs::justify_items,
                &crate::element_attrs::align_self,
                &crate::element_attrs::justify_self,
                &crate::element_attrs::align_content,
                &crate::element_attrs::justify_content,
                &crate::element_attrs::flex_direction,
                &crate::element_attrs::flex_wrap,
                &crate::element_attrs::flex_grow,
                &crate::element_attrs::flex_shrink,
                &crate::element_attrs::flex_basis,
                &crate::element_attrs::visibility,
                &crate::element_attrs::transform,
                &crate::element_attrs::transation,
                &crate::element_attrs::rotation,
                &crate::element_attrs::scale,
                &crate::element_attrs::text_color,
                &crate::element_attrs::font_size,
                &crate::element_attrs::text_linebreak,
                &crate::element_attrs::text_align,
                &crate::element_attrs::font,
                $(&$prop,)*
            ];
        }
    };
    ($(#[$attr:meta])*$name:ident) => {
        impl_element_type_base!($( #[$attr] )*$name,);
    }
}