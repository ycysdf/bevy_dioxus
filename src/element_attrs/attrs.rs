#![allow(non_camel_case_types)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::ui;
use bevy::ui::widget::UiImageSize;

use crate::dom_commands::DomAttributeValue;
use crate::ecs_fns::StyleEntityExt;
use crate::element_core::ElementAttr;

use crate::entity_extra_data::EntitiesExtraData;
use crate::tailwind::handle_classes;
use crate::{
    get_element_type, set_text_value, ReflectTextStyledElementType, SetAttrValueContext,
    TextStyledElementType, UiTexture,
};

pub const ATTR_COUNT: u8 = 53;

pub struct class;

impl From<DomAttributeValue> for Option<String> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => Some(value),
            _ => None,
        }
    }
}

impl ElementAttr for class {
    type Value = String;

    const TAG_NAME: &'static str = stringify!(class);

    const INDEX: u8 = 0;

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

    const INDEX: u8 = class::INDEX + 1;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(Name::new(value.into()));
    }
}
/*pub struct cursor;

impl SchemaProp for cursor {
    type Value = todo!();

    const TAG_NAME: &'static str = stringify!(cursor);

    const INDEX: u8 = 1;
    fn set_value(&self, entity_ref: &mut EntityMut, value: impl Into<Self::Value>) {
        todo!()
    }
}*/

pub struct z_index;

impl ElementAttr for z_index {
    type Value = ZIndex;

    const TAG_NAME: &'static str = stringify!(z_index);

    const INDEX: u8 = name::INDEX + 1;

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct background;

impl ElementAttr for background {
    type Value = UiTexture;

    const TAG_NAME: &'static str = stringify!(background);

    const INDEX: u8 = z_index::INDEX + 1;

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

    const INDEX: u8 = background::INDEX + 1;

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

    const INDEX: u8 = border_left::INDEX + 1;

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

    const INDEX: u8 = border_right::INDEX + 1;

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

    const INDEX: u8 = border_top::INDEX + 1;

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

    const INDEX: u8 = border_bottom::INDEX + 1;

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct display;

impl ElementAttr for display {
    type Value = ui::Display;

    const TAG_NAME: &'static str = stringify!(display);

    const INDEX: u8 = border_color::INDEX + 1;

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

    const INDEX: u8 = display::INDEX + 1;
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

    const INDEX: u8 = position_type::INDEX + 1;
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

    const INDEX: u8 = overflow_x::INDEX + 1;
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

    const INDEX: u8 = overflow_y::INDEX + 1;
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

    const INDEX: u8 = direction::INDEX + 1;

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

    const INDEX: u8 = left::INDEX + 1;
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

    const INDEX: u8 = right::INDEX + 1;
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

    const INDEX: u8 = top::INDEX + 1;
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

    const INDEX: u8 = bottom::INDEX + 1;
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

    const INDEX: u8 = width::INDEX + 1;
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

    const INDEX: u8 = height::INDEX + 1;
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

    const INDEX: u8 = min_width::INDEX + 1;
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

    const INDEX: u8 = min_height::INDEX + 1;
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

    const INDEX: u8 = max_width::INDEX + 1;
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

    const INDEX: u8 = max_height::INDEX + 1;
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

    const INDEX: u8 = margin_left::INDEX + 1;
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

    const INDEX: u8 = margin_right::INDEX + 1;
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

    const INDEX: u8 = margin_top::INDEX + 1;
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

    const INDEX: u8 = margin_bottom::INDEX + 1;
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

    const INDEX: u8 = padding_left::INDEX + 1;
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

    const INDEX: u8 = padding_right::INDEX + 1;
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

    const INDEX: u8 = padding_top::INDEX + 1;
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

    const INDEX: u8 = padding_bottom::INDEX + 1;
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

    const INDEX: u8 = aspect_ratio::INDEX + 1;
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

    const INDEX: u8 = align_items::INDEX + 1;
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

    const INDEX: u8 = justify_items::INDEX + 1;
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

    const INDEX: u8 = align_self::INDEX + 1;
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

    const INDEX: u8 = justify_self::INDEX + 1;
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

    const INDEX: u8 = align_content::INDEX + 1;
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

    const INDEX: u8 = justify_content::INDEX + 1;
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

    const INDEX: u8 = flex_direction::INDEX + 1;
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

    const INDEX: u8 = flex_wrap::INDEX + 1;
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

    const INDEX: u8 = flex_grow::INDEX + 1;
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

    const INDEX: u8 = flex_shrink::INDEX + 1;
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

    const INDEX: u8 = flex_basis::INDEX + 1;
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

    const INDEX: u8 = column_gap::INDEX + 1;
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

    const INDEX: u8 = row_gap::INDEX + 1;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct transation;

impl ElementAttr for transation {
    type Value = Vec3;

    const TAG_NAME: &'static str = stringify!(transation);

    const INDEX: u8 = visibility::INDEX + 1;
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

    const INDEX: u8 = transation::INDEX + 1;
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

    const INDEX: u8 = rotation::INDEX + 1;
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

    const INDEX: u8 = scale::INDEX + 1;
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

    const INDEX: u8 = text_color::INDEX + 1;
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

    const INDEX: u8 = font_size::INDEX + 1;
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

    const INDEX: u8 = text_linebreak::INDEX + 1;
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

    const INDEX: u8 = text_align::INDEX + 1;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, |text_schema_type, entity_ref| {
            text_schema_type.set_font(entity_ref, value.clone());
        });
    }
}
