use bevy::reflect::Reflect;

pub use element_input::*;
pub use element_text::*;
pub use element_view::*;

use crate::{
    define_elements, element_core::ElementTypeUnTyped, ElementAttr, ElementCompositeAttr, ElementType,
    ReflectTextStyledElementType, TextStyledElementType,
};

mod element_input;
mod element_text;
mod element_view;

define_elements!(
    #[derive(Reflect,Debug,Clone,Copy)]
    view {
        [attrs]
        [composite_attrs]
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(TextStyledElementType)]
    text {
        [attrs]
        sections
        [composite_attrs]
    }

    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(TextStyledElementType)]
    input {
        [attrs]
        text_value
        [composite_attrs]
    }
);