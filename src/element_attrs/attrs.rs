#![allow(non_camel_case_types)]

use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::ui;
use bevy::ui::widget::UiImageSize;

use crate::{
    set_text_value, SetAttrValueContext,
    TextStyledElementType, UiTexture,
};
use crate::dom_commands::DomAttributeValue;
use crate::ecs_fns::StyleEntityExt;
use crate::element_core::ElementAttr;
use crate::tailwind::handle_classes;

pub struct class;

impl ElementAttr for class {
    type Value = String;

    const TAG_NAME: &'static str = stringify!(class);

    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue) {
        match value {
            DomAttributeValue::Text(value) => {
                handle_classes(context, &value);
                if !context.entity_extra_data().interaction_classes.is_empty()
                    && !context.entity_ref.contains::<Interaction>()
                {
                    context.entity_ref.insert(Interaction::default());
                }
            }
            _ => {}
        }
    }

    fn set_value(&self, _context: &mut SetAttrValueContext, _value: impl Into<Self::Value>) {
        warn!("You should not set a set_value on a Class!")
    }
}

pub struct name;

impl ElementAttr for name {
    type Value = String;

    const TAG_NAME: &'static str = stringify!(name);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(Name::new(value.into()));
    }
}

pub struct z_index;

impl ElementAttr for z_index {
    type Value = ZIndex;

    const TAG_NAME: &'static str = stringify!(z_index);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct background;

impl ElementAttr for background {
    type Value = UiTexture;

    const TAG_NAME: &'static str = stringify!(background);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        match value.into() {
            UiTexture::Color(color) => {
                context.entity_ref.insert(BackgroundColor(color));
                context.entity_ref.remove::<(
                    UiImage,
                    UiImageSize,
                    Handle<TextureAtlas>,
                    UiTextureAtlasImage,
                )>();
            }
            UiTexture::Image {
                color,
                flip_x,
                flip_y,
                image: image_handle,
            } => {
                context.entity_ref.insert((
                    BackgroundColor(color),
                    UiImage {
                        texture: image_handle,
                        flip_y,
                        flip_x,
                    },
                    UiImageSize::default(),
                ));
                context
                    .entity_ref
                    .remove::<(Handle<TextureAtlas>, UiTextureAtlasImage)>();
            }
            UiTexture::Atlas {
                flip_y,
                flip_x,
                color,
                index,
                atlas,
            } => {
                context.entity_ref.insert((
                    BackgroundColor(color),
                    atlas,
                    UiTextureAtlasImage {
                        index,
                        flip_x,
                        flip_y,
                    },
                ));
                context.entity_ref.remove::<(UiImage, UiImageSize)>();
            }
        }
    }
}

pub struct border_left;

impl ElementAttr for border_left {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(border_left);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.border.left = value;
        });
    }
}

pub struct border_right;

impl ElementAttr for border_right {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(border_right);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.border.right = value;
        });
    }
}

pub struct border_top;

impl ElementAttr for border_top {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(border_top);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.border.top = value;
        });
    }
}

pub struct border_bottom;

impl ElementAttr for border_bottom {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(border_bottom);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.border.bottom = value;
        });
    }
}

pub struct border_color;

impl ElementAttr for border_color {
    type Value = BorderColor;

    const TAG_NAME: &'static str = stringify!(border_color);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct display;

impl ElementAttr for display {
    type Value = ui::Display;

    const TAG_NAME: &'static str = stringify!(display);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.display = value.into();
        });
    }
}

pub struct position_type;

impl ElementAttr for position_type {
    type Value = PositionType;

    const TAG_NAME: &'static str = stringify!(position_type);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.position_type = value.into();
        });
    }
}

pub struct overflow_x;

impl ElementAttr for overflow_x {
    type Value = OverflowAxis;

    const TAG_NAME: &'static str = stringify!(overflow_x);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.overflow.x = value;
        });
    }
}

pub struct overflow_y;

impl ElementAttr for overflow_y {
    type Value = OverflowAxis;

    const TAG_NAME: &'static str = stringify!(overflow_y);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.overflow.y = value;
        });
    }
}

pub struct direction;

impl ElementAttr for direction {
    type Value = Direction;

    const TAG_NAME: &'static str = stringify!(direction);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.direction = value.into();
        });
    }
}

pub struct left;

impl ElementAttr for left {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(left);

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.left = value.into();
        });
    }
}

pub struct right;

impl ElementAttr for right {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(right);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.right = value.into();
        });
    }
}

pub struct top;

impl ElementAttr for top {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(top);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.top = value.into();
        });
    }
}

pub struct bottom;

impl ElementAttr for bottom {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(bottom);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.bottom = value.into();
        });
    }
}

pub struct width;

impl ElementAttr for width {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(width);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.width = value.into();
        });
    }
}

pub struct height;

impl ElementAttr for height {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(height);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.height = value.into();
        });
    }
}

pub struct min_width;

impl ElementAttr for min_width {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(min_width);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.min_width = value.into();
        });
    }
}

pub struct min_height;

impl ElementAttr for min_height {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(min_height);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.min_height = value.into();
        });
    }
}

pub struct max_width;

impl ElementAttr for max_width {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(max_width);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.max_width = value.into();
        });
    }
}

pub struct max_height;

impl ElementAttr for max_height {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(max_height);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.max_height = value.into();
        });
    }
}

pub struct margin_left;

impl ElementAttr for margin_left {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(margin_left);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.margin.left = value;
        });
    }
}

pub struct margin_right;

impl ElementAttr for margin_right {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(margin_right);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.margin.right = value;
        });
    }
}

pub struct margin_top;

impl ElementAttr for margin_top {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(margin_top);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.margin.top = value;
        });
    }
}

pub struct margin_bottom;

impl ElementAttr for margin_bottom {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(margin_bottom);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.margin.bottom = value;
        });
    }
}

pub struct padding_left;

impl ElementAttr for padding_left {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(padding_left);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.padding.left = value;
        });
    }
}

pub struct padding_right;

impl ElementAttr for padding_right {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(padding_right);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.padding.right = value;
        });
    }
}

pub struct padding_top;

impl ElementAttr for padding_top {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(padding_top);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.padding.top = value;
        });
    }
}

pub struct padding_bottom;

impl ElementAttr for padding_bottom {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(padding_bottom);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.padding.bottom = value;
        });
    }
}

pub struct aspect_ratio;

impl ElementAttr for aspect_ratio {
    type Value = Option<f32>;

    const TAG_NAME: &'static str = stringify!(aspect_ratio);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.aspect_ratio = value.into();
        });
    }
}

pub struct align_items;

impl ElementAttr for align_items {
    type Value = AlignItems;

    const TAG_NAME: &'static str = stringify!(align_items);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.align_items = value.into();
        });
    }
}

pub struct justify_items;

impl ElementAttr for justify_items {
    type Value = JustifyItems;

    const TAG_NAME: &'static str = stringify!(justify_items);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.justify_items = value.into();
        });
    }
}

pub struct align_self;

impl ElementAttr for align_self {
    type Value = AlignSelf;

    const TAG_NAME: &'static str = stringify!(align_self);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.align_self = value.into();
        });
    }
}

pub struct justify_self;

impl ElementAttr for justify_self {
    type Value = JustifySelf;

    const TAG_NAME: &'static str = stringify!(justify_self);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.justify_self = value.into();
        });
    }
}

pub struct align_content;

impl ElementAttr for align_content {
    type Value = AlignContent;

    const TAG_NAME: &'static str = stringify!(align_content);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.align_content = value.into();
        });
    }
}

pub struct justify_content;

impl ElementAttr for justify_content {
    type Value = JustifyContent;

    const TAG_NAME: &'static str = stringify!(justify_content);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.justify_content = value.into();
        });
    }
}

pub struct flex_direction;

impl ElementAttr for flex_direction {
    type Value = FlexDirection;

    const TAG_NAME: &'static str = stringify!(flex_direction);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_direction = value.into();
        });
    }
}

pub struct flex_wrap;

impl ElementAttr for flex_wrap {
    type Value = FlexWrap;

    const TAG_NAME: &'static str = stringify!(flex_wrap);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_wrap = value.into();
        });
    }
}

pub struct flex_grow;

impl ElementAttr for flex_grow {
    type Value = f32;

    const TAG_NAME: &'static str = stringify!(flex_grow);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_grow = value.into();
        });
    }
}

pub struct flex_shrink;

impl ElementAttr for flex_shrink {
    type Value = f32;

    const TAG_NAME: &'static str = stringify!(flex_shrink);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_shrink = value.into();
        });
    }
}

pub struct flex_basis;

impl ElementAttr for flex_basis {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(flex_basis);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_basis = value.into();
        });
    }
}

pub struct column_gap;

impl ElementAttr for column_gap {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(column_gap);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.column_gap = value.into();
        });
    }
}

pub struct row_gap;

impl ElementAttr for row_gap {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(row_gap);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.row_gap = value.into();
        });
    }
}

pub struct visibility;

impl ElementAttr for visibility {
    type Value = Visibility;

    const TAG_NAME: &'static str = stringify!(visibility);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct transation;

impl ElementAttr for transation {
    type Value = Vec3;

    const TAG_NAME: &'static str = stringify!(transation);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut tf) = context.entity_ref.get_mut::<Transform>() {
            tf.translation = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

pub struct rotation;

impl ElementAttr for rotation {
    type Value = Quat;

    const TAG_NAME: &'static str = stringify!(rotation);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut tf) = context.entity_ref.get_mut::<Transform>() {
            tf.rotation = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

pub struct scale;

impl ElementAttr for scale {
    type Value = Vec3;

    const TAG_NAME: &'static str = stringify!(scale);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut tf) = context.entity_ref.get_mut::<Transform>() {
            tf.scale = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

pub struct text_color;

impl ElementAttr for text_color {
    type Value = Color;

    const TAG_NAME: &'static str = stringify!(text_color);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_text_color(entity_ref, value);
        });
    }
}

pub struct font_size;

impl ElementAttr for font_size {
    type Value = f32;

    const TAG_NAME: &'static str = stringify!(font_size);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_font_size(entity_ref, value);
        });
    }
}

pub struct text_linebreak;

impl ElementAttr for text_linebreak {
    type Value = BreakLineOn;

    const TAG_NAME: &'static str = stringify!(text_linebreak);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_text_linebreak(entity_ref, value);
        });
    }
}

pub struct text_align;

impl ElementAttr for text_align {
    type Value = TextAlignment;

    const TAG_NAME: &'static str = stringify!(text_align);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_text_align(entity_ref, value);
        });
    }
}

pub struct font;

impl ElementAttr for font {
    type Value = Handle<Font>;

    const TAG_NAME: &'static str = stringify!(font);
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_font(entity_ref, value.clone());
        });
    }
}
