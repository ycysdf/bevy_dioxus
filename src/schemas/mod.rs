pub use schema_attribute_values::*;
pub use schema_input::*;
pub use schema_text::*;
pub use schema_view::*;

use crate::schema_core::SchemaTypeUnTyped;

mod schema_attribute_values;
pub mod schema_events;
mod schema_input;
pub mod schema_props;
mod schema_text;
mod schema_view;

pub fn try_get_schema_type(name: &str) -> Option<&'static dyn SchemaTypeUnTyped> {
    match name {
        stringify!(view) => Some(&view),
        stringify!(text) => Some(&text),
        stringify!(input) => Some(&input),
        // stringify!(svg) => Some(&svg),
        _ => None,
    }
}

pub fn get_schema_type(name: &str) -> &'static dyn SchemaTypeUnTyped {
    try_get_schema_type(name).expect(&format!("No Found SchemaType by {:#?}", name))
}


#[macro_export]
macro_rules! common_props_define {
    () => {
        pub const class: crate::DioxusAttributeDescription =
            crate::schema_props::class::ATTRIBUTE_DESCRIPTION;
        pub const name: crate::DioxusAttributeDescription =
            crate::schema_props::name::ATTRIBUTE_DESCRIPTION;
        pub const z_index: crate::DioxusAttributeDescription =
            crate::schema_props::z_index::ATTRIBUTE_DESCRIPTION;
        pub const background: crate::DioxusAttributeDescription =
            crate::schema_props::background::ATTRIBUTE_DESCRIPTION;
        pub const border: crate::DioxusAttributeDescription =
            crate::schema_props::border::ATTRIBUTE_DESCRIPTION;
        pub const border_color: crate::DioxusAttributeDescription =
            crate::schema_props::border_color::ATTRIBUTE_DESCRIPTION;
        pub const display: crate::DioxusAttributeDescription =
            crate::schema_props::display::ATTRIBUTE_DESCRIPTION;
        pub const position_type: crate::DioxusAttributeDescription =
            crate::schema_props::position_type::ATTRIBUTE_DESCRIPTION;
        pub const overflow: crate::DioxusAttributeDescription =
            crate::schema_props::overflow::ATTRIBUTE_DESCRIPTION;
        pub const direction: crate::DioxusAttributeDescription =
            crate::schema_props::direction::ATTRIBUTE_DESCRIPTION;
        pub const left: crate::DioxusAttributeDescription =
            crate::schema_props::left::ATTRIBUTE_DESCRIPTION;
        pub const right: crate::DioxusAttributeDescription =
            crate::schema_props::right::ATTRIBUTE_DESCRIPTION;
        pub const top: crate::DioxusAttributeDescription =
            crate::schema_props::top::ATTRIBUTE_DESCRIPTION;
        pub const bottom: crate::DioxusAttributeDescription =
            crate::schema_props::bottom::ATTRIBUTE_DESCRIPTION;
        pub const width: crate::DioxusAttributeDescription =
            crate::schema_props::width::ATTRIBUTE_DESCRIPTION;
        pub const height: crate::DioxusAttributeDescription =
            crate::schema_props::height::ATTRIBUTE_DESCRIPTION;
        pub const min_width: crate::DioxusAttributeDescription =
            crate::schema_props::min_width::ATTRIBUTE_DESCRIPTION;
        pub const min_height: crate::DioxusAttributeDescription =
            crate::schema_props::min_height::ATTRIBUTE_DESCRIPTION;
        pub const max_width: crate::DioxusAttributeDescription =
            crate::schema_props::max_width::ATTRIBUTE_DESCRIPTION;
        pub const max_height: crate::DioxusAttributeDescription =
            crate::schema_props::max_height::ATTRIBUTE_DESCRIPTION;
        pub const margin: crate::DioxusAttributeDescription =
            crate::schema_props::margin::ATTRIBUTE_DESCRIPTION;
        pub const padding: crate::DioxusAttributeDescription =
            crate::schema_props::padding::ATTRIBUTE_DESCRIPTION;
        pub const aspect_ratio: crate::DioxusAttributeDescription =
            crate::schema_props::aspect_ratio::ATTRIBUTE_DESCRIPTION;
        pub const align_items: crate::DioxusAttributeDescription =
            crate::schema_props::align_items::ATTRIBUTE_DESCRIPTION;
        pub const justify_items: crate::DioxusAttributeDescription =
            crate::schema_props::justify_items::ATTRIBUTE_DESCRIPTION;
        pub const align_self: crate::DioxusAttributeDescription =
            crate::schema_props::align_self::ATTRIBUTE_DESCRIPTION;
        pub const justify_self: crate::DioxusAttributeDescription =
            crate::schema_props::justify_self::ATTRIBUTE_DESCRIPTION;
        pub const align_content: crate::DioxusAttributeDescription =
            crate::schema_props::align_content::ATTRIBUTE_DESCRIPTION;
        pub const justify_content: crate::DioxusAttributeDescription =
            crate::schema_props::justify_content::ATTRIBUTE_DESCRIPTION;
        pub const flex_direction: crate::DioxusAttributeDescription =
            crate::schema_props::flex_direction::ATTRIBUTE_DESCRIPTION;
        pub const flex_wrap: crate::DioxusAttributeDescription =
            crate::schema_props::flex_wrap::ATTRIBUTE_DESCRIPTION;
        pub const flex_grow: crate::DioxusAttributeDescription =
            crate::schema_props::flex_grow::ATTRIBUTE_DESCRIPTION;
        pub const flex_shrink: crate::DioxusAttributeDescription =
            crate::schema_props::flex_shrink::ATTRIBUTE_DESCRIPTION;
        pub const flex_basis: crate::DioxusAttributeDescription =
            crate::schema_props::flex_basis::ATTRIBUTE_DESCRIPTION;
        pub const visibility: crate::DioxusAttributeDescription =
            crate::schema_props::visibility::ATTRIBUTE_DESCRIPTION;
        pub const transform: crate::DioxusAttributeDescription =
            crate::schema_props::transform::ATTRIBUTE_DESCRIPTION;
        pub const transation: crate::DioxusAttributeDescription =
            crate::schema_props::transation::ATTRIBUTE_DESCRIPTION;
        pub const rotation: crate::DioxusAttributeDescription =
            crate::schema_props::rotation::ATTRIBUTE_DESCRIPTION;
        pub const scale: crate::DioxusAttributeDescription =
            crate::schema_props::scale::ATTRIBUTE_DESCRIPTION;
        pub const text_color: crate::DioxusAttributeDescription =
            crate::schema_props::text_color::ATTRIBUTE_DESCRIPTION;
        pub const font_size: crate::DioxusAttributeDescription =
            crate::schema_props::font_size::ATTRIBUTE_DESCRIPTION;
    };
}

#[macro_export]
macro_rules! impl_schema_type_base {
    ($name:ident,$($prop:ident),*) => {
        use crate::schema_core::{SchemaProp};

        pub struct $name;
        impl $name {
            crate::common_props_define!();
            $(
                pub const $prop: crate::DioxusAttributeDescription = $prop::ATTRIBUTE_DESCRIPTION;
            )*
        }
        impl crate::SchemaTypeBase for $name {
            const TAG_NAME: &'static str = stringify!($name);
            const PROPS: &'static [&'static dyn crate::SchemaPropUntyped] = &[
                &crate::schema_props::class,
                &crate::schema_props::name,
                &crate::schema_props::z_index,
                &crate::schema_props::background,
                &crate::schema_props::border,
                &crate::schema_props::border_color,
                &crate::schema_props::display,
                &crate::schema_props::position_type,
                &crate::schema_props::overflow,
                &crate::schema_props::direction,
                &crate::schema_props::left,
                &crate::schema_props::right,
                &crate::schema_props::top,
                &crate::schema_props::bottom,
                &crate::schema_props::width,
                &crate::schema_props::height,
                &crate::schema_props::min_width,
                &crate::schema_props::min_height,
                &crate::schema_props::max_width,
                &crate::schema_props::max_height,
                &crate::schema_props::margin,
                &crate::schema_props::padding,
                &crate::schema_props::aspect_ratio,
                &crate::schema_props::align_items,
                &crate::schema_props::justify_items,
                &crate::schema_props::align_self,
                &crate::schema_props::justify_self,
                &crate::schema_props::align_content,
                &crate::schema_props::justify_content,
                &crate::schema_props::flex_direction,
                &crate::schema_props::flex_wrap,
                &crate::schema_props::flex_grow,
                &crate::schema_props::flex_shrink,
                &crate::schema_props::flex_basis,
                &crate::schema_props::visibility,
                &crate::schema_props::transform,
                &crate::schema_props::transation,
                &crate::schema_props::rotation,
                &crate::schema_props::scale,
                &crate::schema_props::text_color,
                &crate::schema_props::font_size,
                $(&$prop,)*
            ];
        }
    };
    ($name:ident) => {
        impl_schema_type_base!($name,);
    }
}

// Val
// pub const row_gap: AttributeDescription = AttributeDescription("row_gap", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // Val
// pub const column_gap: AttributeDescription = AttributeDescription("column_gap", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // Val
// pub const grid_auto_flow: AttributeDescription = AttributeDescription("grid_auto_flow", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // GridAutoFlow
// pub const grid_template_rows: AttributeDescription = AttributeDescription("grid_template_rows", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // Vec<RepeatedGridTrack>
// pub const grid_template_columns: AttributeDescription = AttributeDescription("grid_template_columns", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // Vec<RepeatedGridTrack>
// pub const grid_auto_rows: AttributeDescription = AttributeDescription("grid_auto_rows", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // Vec<GridTrack>
// pub const grid_auto_columns: AttributeDescription = AttributeDescription("grid_auto_columns", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // Vec<GridTrack>
// pub const grid_row: AttributeDescription = AttributeDescription("grid_row", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// });
// // GridPlacement
// pub const grid_column: AttributeDescription = AttributeDescription("grid_column", None, false, |entity_ref, value,_| match value {
//     DomAttributeValue::Text(_) => {}
//     DomAttributeValue::Any(_) => {}
//     _ => {}
// }); // GridPlacement*/
