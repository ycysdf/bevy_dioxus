use bevy::prelude::Transform;
use bevy::ui::UiRect;

use crate::{element_attrs, ElementCompositeAttr, OptionalTransform};
use crate::{
    ElementAttrUntyped, SetAttrValueContext, StyleEntityExt,
    UiOptionalRect,
};
use crate::OptionalValue;

impl OptionalValue for UiOptionalRect {
    fn get_count(&self) -> u8 {
        4
    }
    fn get_valid_indices_bits(&self) -> u8 {
        let mut r = 0u8;
        if self.left.is_some() {
            r |= 1 << 0;
        }
        if self.right.is_some() {
            r |= 1 << 1;
        }
        if self.top.is_some() {
            r |= 1 << 2;
        }
        if self.bottom.is_some() {
            r |= 1 << 3;
        }
        r
    }
}

pub struct margin;

impl ElementCompositeAttr for margin {
    type Value = UiOptionalRect;

    const TAG_NAME: &'static str = stringify!(margin);

    const ATTRS: &'static [&'static dyn ElementAttrUntyped] = &[
        &element_attrs::margin_left,
        &element_attrs::margin_right,
        &element_attrs::margin_top,
        &element_attrs::margin_bottom,
    ];

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.margin = UiRect {
                left: value.left.unwrap_or(style.margin.left),
                right: value.right.unwrap_or(style.margin.right),
                top: value.top.unwrap_or(style.margin.top),
                bottom: value.bottom.unwrap_or(style.margin.bottom),
            };
        });
    }
}

pub struct padding;

impl ElementCompositeAttr for padding {
    type Value = UiOptionalRect;

    const TAG_NAME: &'static str = stringify!(padding);

    const ATTRS: &'static [&'static dyn ElementAttrUntyped] = &[
        &element_attrs::padding_left,
        &element_attrs::padding_right,
        &element_attrs::padding_top,
        &element_attrs::padding_bottom,
    ];

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.padding = UiRect {
                left: value.left.unwrap_or(style.padding.left),
                right: value.right.unwrap_or(style.padding.right),
                top: value.top.unwrap_or(style.padding.top),
                bottom: value.bottom.unwrap_or(style.padding.bottom),
            };
        });
    }
}

pub struct border;

impl ElementCompositeAttr for border {
    type Value = UiOptionalRect;

    const TAG_NAME: &'static str = stringify!(border);

    const ATTRS: &'static [&'static dyn ElementAttrUntyped] = &[
        &element_attrs::border_left,
        &element_attrs::border_right,
        &element_attrs::border_top,
        &element_attrs::border_bottom,
    ];

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.border = UiRect {
                left: value.left.unwrap_or(style.border.left),
                right: value.right.unwrap_or(style.border.right),
                top: value.top.unwrap_or(style.border.top),
                bottom: value.bottom.unwrap_or(style.border.bottom),
            };
        });
    }
}

impl OptionalValue for OptionalTransform {
    fn get_count(&self) -> u8 {
        3
    }

    fn get_valid_indices_bits(&self) -> u8 {
        let mut r = 0u8;
        if self.translation.is_some() {
            r |= 1 << 0;
        }
        if self.rotation.is_some() {
            r |= 1 << 1;
        }
        if self.scale.is_some() {
            r |= 1 << 2;
        }
        r
    }
}

pub struct transform;

impl ElementCompositeAttr for transform {
    type Value = OptionalTransform;

    const TAG_NAME: &'static str = stringify!(transform);
    const ATTRS: &'static [&'static dyn ElementAttrUntyped] = &[
        &element_attrs::transation,
        &element_attrs::rotation,
        &element_attrs::scale,
    ];

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        let Some(t) = context.entity_ref.get::<Transform>() else {
            return;
        };
        context.entity_ref.insert(Transform {
            translation: value.translation.unwrap_or(t.translation),
            rotation: value.rotation.unwrap_or(t.rotation),
            scale: value.scale.unwrap_or(t.scale),
        });
    }
}
