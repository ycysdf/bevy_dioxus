#![allow(non_camel_case_types)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::ui;
use bevy::ui::widget::UiImageSize;

use crate::{PropValue, schemas, SchemaTypeBase, SmallBox};
use crate::{OptionalOverflow, SetAttrValueContext, Texture, UiOptionalRect};
use crate::dom_commands::DomAttributeValue;
use crate::ecs_fns::StyleEntityExt;
use crate::schema_core::SchemaProp;
use crate::smallbox::S1;
use crate::tailwind::handle_classes;

pub const COMMON_PROPS_COUNT: u8 = 44;

pub struct class;

impl From<DomAttributeValue> for Option<String> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => Some(value),
            _ => None,
        }
    }
}

impl SchemaProp for class {
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

impl SchemaProp for name {
    type Value = String;

    const TAG_NAME: &'static str = stringify!(name);

    const INDEX: u8 = 1;
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

impl SchemaProp for z_index {
    type Value = ZIndex;

    const TAG_NAME: &'static str = stringify!(z_index);

    const INDEX: u8 = 2;

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct background;

impl SchemaProp for background {
    type Value = Texture;

    const TAG_NAME: &'static str = stringify!(background);

    const INDEX: u8 = 3;

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        match value.into() {
            Texture::Color(color) => {
                context.entity_ref.insert(BackgroundColor(color));
                context.entity_ref.remove::<(
                    UiImage,
                    UiImageSize,
                    Handle<TextureAtlas>,
                    UiTextureAtlasImage,
                )>();
            }
            Texture::Image {
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
            Texture::Atlas {
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

pub struct border;

impl SchemaProp for border {
    type Value = UiOptionalRect;

    const TAG_NAME: &'static str = stringify!(border);

    const INDEX: u8 = 4;

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

pub struct border_color;

impl SchemaProp for border_color {
    type Value = BorderColor;

    const TAG_NAME: &'static str = stringify!(border_color);

    const INDEX: u8 = 5;

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct display;

impl SchemaProp for display {
    type Value = ui::Display;

    const TAG_NAME: &'static str = stringify!(display);

    const INDEX: u8 = 6;

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.display = value.into();
        });
    }
}

pub struct position_type;

impl SchemaProp for position_type {
    type Value = PositionType;

    const TAG_NAME: &'static str = stringify!(position_type);

    const INDEX: u8 = 7;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.position_type = value.into();
        });
    }
}

pub struct overflow;

impl SchemaProp for overflow {
    type Value = OptionalOverflow;

    const TAG_NAME: &'static str = stringify!(overflow);

    const INDEX: u8 = 8;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            let value = value.into();
            style.overflow = Overflow {
                x: value.x.unwrap_or(style.overflow.x),
                y: value.y.unwrap_or(style.overflow.y),
            };
        });
    }
}

pub struct direction;

impl SchemaProp for direction {
    type Value = Direction;

    const TAG_NAME: &'static str = stringify!(direction);

    const INDEX: u8 = 9;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.direction = value.into();
        });
    }
}

pub struct left;

impl SchemaProp for left {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(left);

    const INDEX: u8 = 10;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.left = value.into();
        });
    }
}

pub struct right;

impl SchemaProp for right {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(right);

    const INDEX: u8 = 11;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.right = value.into();
        });
    }
}

pub struct top;

impl SchemaProp for top {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(top);

    const INDEX: u8 = 12;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.top = value.into();
        });
    }
}

pub struct bottom;

impl SchemaProp for bottom {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(bottom);

    const INDEX: u8 = 13;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.bottom = value.into();
        });
    }
}

pub struct width;

impl SchemaProp for width {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(width);

    const INDEX: u8 = 14;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.width = value.into();
        });
    }
}

pub struct height;

impl SchemaProp for height {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(height);

    const INDEX: u8 = 15;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.height = value.into();
        });
    }
}

pub struct min_width;

impl SchemaProp for min_width {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(min_width);

    const INDEX: u8 = 16;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.min_width = value.into();
        });
    }
}

pub struct min_height;

impl SchemaProp for min_height {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(min_height);

    const INDEX: u8 = 17;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.min_height = value.into();
        });
    }
}

pub struct max_width;

impl SchemaProp for max_width {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(max_width);

    const INDEX: u8 = 18;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.max_width = value.into();
        });
    }
}

pub struct max_height;

impl SchemaProp for max_height {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(max_height);

    const INDEX: u8 = 19;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.max_height = value.into();
        });
    }
}

pub struct margin;

impl SchemaProp for margin {
    type Value = UiOptionalRect;

    const TAG_NAME: &'static str = stringify!(margin);

    const INDEX: u8 = 20;
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

impl SchemaProp for padding {
    type Value = UiOptionalRect;

    const TAG_NAME: &'static str = stringify!(padding);

    const INDEX: u8 = 21;
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

    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn PropValue, S1>) {
        if let Ok(value) = value.downcast::<Self::Value>() {
            self.set_value(context, value.into_inner());
        }
    }
}

pub struct aspect_ratio;

impl SchemaProp for aspect_ratio {
    type Value = Option<f32>;

    const TAG_NAME: &'static str = stringify!(aspect_ratio);

    const INDEX: u8 = 22;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.aspect_ratio = value.into();
        });
    }
}

pub struct align_items;

impl SchemaProp for align_items {
    type Value = AlignItems;

    const TAG_NAME: &'static str = stringify!(align_items);

    const INDEX: u8 = 23;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.align_items = value.into();
        });
    }
}

pub struct justify_items;

impl SchemaProp for justify_items {
    type Value = JustifyItems;

    const TAG_NAME: &'static str = stringify!(justify_items);

    const INDEX: u8 = 24;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.justify_items = value.into();
        });
    }
}

pub struct align_self;

impl SchemaProp for align_self {
    type Value = AlignSelf;

    const TAG_NAME: &'static str = stringify!(align_self);

    const INDEX: u8 = 25;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.align_self = value.into();
        });
    }
}

pub struct justify_self;

impl SchemaProp for justify_self {
    type Value = JustifySelf;

    const TAG_NAME: &'static str = stringify!(justify_self);

    const INDEX: u8 = 26;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.justify_self = value.into();
        });
    }
}

pub struct align_content;

impl SchemaProp for align_content {
    type Value = AlignContent;

    const TAG_NAME: &'static str = stringify!(align_content);

    const INDEX: u8 = 27;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.align_content = value.into();
        });
    }
}

pub struct justify_content;

impl SchemaProp for justify_content {
    type Value = JustifyContent;

    const TAG_NAME: &'static str = stringify!(justify_content);

    const INDEX: u8 = 28;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.justify_content = value.into();
        });
    }
}

pub struct flex_direction;

impl SchemaProp for flex_direction {
    type Value = FlexDirection;

    const TAG_NAME: &'static str = stringify!(flex_direction);

    const INDEX: u8 = 29;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_direction = value.into();
        });
    }
}

pub struct flex_wrap;

impl SchemaProp for flex_wrap {
    type Value = FlexWrap;

    const TAG_NAME: &'static str = stringify!(flex_wrap);

    const INDEX: u8 = 30;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_wrap = value.into();
        });
    }
}

pub struct flex_grow;

impl SchemaProp for flex_grow {
    type Value = f32;

    const TAG_NAME: &'static str = stringify!(flex_grow);

    const INDEX: u8 = 31;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_grow = value.into();
        });
    }
}

pub struct flex_shrink;

impl SchemaProp for flex_shrink {
    type Value = f32;

    const TAG_NAME: &'static str = stringify!(flex_shrink);

    const INDEX: u8 = 32;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_shrink = value.into();
        });
    }
}

pub struct flex_basis;

impl SchemaProp for flex_basis {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(flex_basis);

    const INDEX: u8 = 33;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.flex_basis = value.into();
        });
    }
}

pub struct column_gap;

impl SchemaProp for column_gap {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(column_gap);

    const INDEX: u8 = 34;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.column_gap = value.into();
        });
    }
}

pub struct row_gap;

impl SchemaProp for row_gap {
    type Value = Val;

    const TAG_NAME: &'static str = stringify!(row_gap);

    const INDEX: u8 = 35;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.try_set_style(|style| {
            style.row_gap = value.into();
        });
    }
}

pub struct visibility;

impl SchemaProp for visibility {
    type Value = Visibility;

    const TAG_NAME: &'static str = stringify!(visibility);

    const INDEX: u8 = 35;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct transform;

impl SchemaProp for transform {
    type Value = Transform;

    const TAG_NAME: &'static str = stringify!(transform);

    const INDEX: u8 = 36;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        context.entity_ref.insert(value.into());
    }
}

pub struct transation;

impl SchemaProp for transation {
    type Value = Vec3;

    const TAG_NAME: &'static str = stringify!(transation);

    const INDEX: u8 = 37;
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

impl SchemaProp for rotation {
    type Value = Quat;

    const TAG_NAME: &'static str = stringify!(rotation);

    const INDEX: u8 = 38;
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

impl SchemaProp for scale {
    type Value = Vec3;

    const TAG_NAME: &'static str = stringify!(scale);

    const INDEX: u8 = 39;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        if let Some(mut tf) = context.entity_ref.get_mut::<Transform>() {
            tf.scale = value;
        } else {
            warn!("no found Transform component!");
        }
    }
}

fn set_text_value(context: &mut SetAttrValueContext, mut f: impl FnMut(Mut<Text>)) {
    let mut entity_set_value = |entity_ref: &mut EntityMut| {
        if let Some(text) = entity_ref.get_mut::<Text>() {
            f(text)
        }
    };
    if context.entity_extra_data().schema_name != schemas::text::NAME {
        let children = context
            .entity_ref
            .get_mut::<Children>()
            .map(|c| {
                c.into_iter()
                    .filter(|e| {
                        context
                            .entities_extra_data
                            .get(*e)
                            .is_some_and(|n| n.schema_name == schemas::text::NAME)
                    })
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        for entity in children {
            context.entity_ref.world_scope(|world| {
                entity_set_value(&mut world.entity_mut(entity));
            })
        }
    } else {
        entity_set_value(context.entity_ref);
    }
}

pub struct text_color;

impl SchemaProp for text_color {
    type Value = Color;

    const TAG_NAME: &'static str = stringify!(text_color);

    const INDEX: u8 = 40;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, move |mut text| {
            for section in text.sections.iter_mut() {
                section.style.color = value;
            }
        });
    }
}

pub struct font_size;

impl SchemaProp for font_size {
    type Value = f32;

    const TAG_NAME: &'static str = stringify!(font_size);

    const INDEX: u8 = 41;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, move |mut text| {
            for section in text.sections.iter_mut() {
                section.style.font_size = value;
            }
        });
    }
}

pub struct text_linebreak_behavior;

impl SchemaProp for text_linebreak_behavior {
    type Value = BreakLineOn;

    const TAG_NAME: &'static str = stringify!(text_color);

    const INDEX: u8 = 42;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, move |mut text| {
            text.linebreak_behavior = value;
        });
    }
}


pub struct text_align;

impl SchemaProp for text_align {
    type Value = TextAlignment;

    const TAG_NAME: &'static str = stringify!(text_align);

    const INDEX: u8 = 43;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let value = value.into();
        set_text_value(context, move |mut text| {
            text.alignment = value;
        });
    }
}
