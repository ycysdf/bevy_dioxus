use bevy::{
    ecs::world::EntityMut,
    prelude::{Children, Entity},
    reflect::reflect_trait,
};

pub use element_input::*;
pub use element_text::*;
pub use element_view::*;

use crate::element_attrs;
use crate::{element_core::ElementTypeUnTyped, ElementAttr, SetAttrValueContext};
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
macro_rules! attr_description_define{
    ($($attr:ident),*) =>{
        $(pub const $attr: crate::DioxusAttributeDescription =
            crate::element_attrs::$attr::ATTRIBUTE_DESCRIPTION;)*
    }
}

#[macro_export]
macro_rules! common_attrs_define {
    () => {
        crate::attr_description_define!(
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
    };
}

#[macro_export]
macro_rules! common_composite_attrs_define {
    () => {
        crate::attr_description_define!(margin, padding, border, transform);
    };
}

#[macro_export]
macro_rules! impl_element_type_base {
    ($(#[$m_attr:meta])*$name:ident,$($attr:ident),* & $($comopsite_attr:ident),*) => {
        use crate::{ElementCompositeAttr,ElementCompositeAttrUntyped,ElementAttr};
        use crate::element_attrs as e;
        $( #[$m_attr] )*
        pub struct $name;
        impl $name {
            crate::common_attrs_define!();
            crate::common_composite_attrs_define!();
            $(
                pub const $attr: crate::DioxusAttributeDescription = $attr::ATTRIBUTE_DESCRIPTION;
            )*
            $(
                pub const $comopsite_attr: crate::DioxusAttributeDescription = $comopsite_attr::ATTRIBUTE_DESCRIPTION;
            )*
        }
        impl crate::ElementTypeBase for $name {
            const TAG_NAME: &'static str = stringify!($name);
            const ATTRS: &'static [&'static dyn crate::ElementAttrUntyped] = &[
                &e::class,
                &e::name,
                &e::z_index,
                &e::background,
                &e::border_left,
                &e::border_right,
                &e::border_top,
                &e::border_bottom,
                &e::border_color,
                &e::display,
                &e::position_type,
                &e::overflow_x,
                &e::overflow_y,
                &e::direction,
                &e::left,
                &e::right,
                &e::top,
                &e::bottom,
                &e::width,
                &e::height,
                &e::min_width,
                &e::min_height,
                &e::max_width,
                &e::max_height,
                &e::margin_left,
                &e::margin_right,
                &e::margin_top,
                &e::margin_bottom,
                &e::padding_left,
                &e::padding_right,
                &e::padding_top,
                &e::padding_bottom,
                &e::aspect_ratio,
                &e::align_items,
                &e::justify_items,
                &e::align_self,
                &e::justify_self,
                &e::align_content,
                &e::justify_content,
                &e::flex_direction,
                &e::flex_wrap,
                &e::flex_grow,
                &e::flex_shrink,
                &e::flex_basis,
                &e::visibility,
                &e::transation,
                &e::rotation,
                &e::scale,
                &e::text_color,
                &e::font_size,
                &e::text_linebreak,
                &e::text_align,
                &e::font,
                $(&$attr,)*
            ];

            const COMPOSITE_ATTRS: &'static [&'static dyn ElementCompositeAttrUntyped] = &[
                &e::margin, &e::padding, &e::border, &e::transform
            ];

        }
    };
    ($(#[$m_attr:meta])*$name:ident,$($attr:ident),*) => {
        impl_element_type_base!($( #[$m_attr] )*$name,$($attr),*&);
    };
    ($(#[$m_attr:meta])*$name:ident) => {
        impl_element_type_base!($( #[$m_attr] )*$name,&);
    }
}
