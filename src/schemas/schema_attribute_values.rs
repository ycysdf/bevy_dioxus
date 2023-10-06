use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use serde::{Deserialize, Serialize};

use crate::dom_commands::DomAttributeValue;
use crate::entity_extra_data::{EntitiesExtraData, EntityExtraData};
use crate::tailwind::{parse_color, parse_size_val};

pub struct SetAttrValueContext<'w> {
    pub entities_extra_data: &'w mut EntitiesExtraData,
    pub entity_ref: &'w mut EntityMut<'w>,
}

impl<'w> SetAttrValueContext<'w> {
    pub fn entity_extra_data(&mut self) -> &mut EntityExtraData {
        self.entities_extra_data.get_mut(&self.entity_ref.id()).unwrap()
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Reflect, Serialize, Deserialize)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct OptionalOverflow {
    pub x: Option<OverflowAxis>,
    pub y: Option<OverflowAxis>,
}

#[derive(Copy, Clone, Default, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub struct UiOptionalRect {
    pub left: Option<Val>,
    pub right: Option<Val>,
    pub top: Option<Val>,
    pub bottom: Option<Val>,
}

impl UiOptionalRect {
    pub fn all(val: Val) -> Self {
        Self {
            left: Some(val),
            right: Some(val),
            top: Some(val),
            bottom: Some(val),
        }
    }

    pub const fn new(left: Val, right: Val, top: Val, bottom: Val) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            top: Some(top),
            bottom: Some(bottom),
        }
    }

    pub const fn px(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Px(left)),
            right: Some(Val::Px(right)),
            top: Some(Val::Px(top)),
            bottom: Some(Val::Px(bottom)),
        }
    }

    pub const fn percent(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Some(Val::Percent(left)),
            right: Some(Val::Percent(right)),
            top: Some(Val::Percent(top)),
            bottom: Some(Val::Percent(bottom)),
        }
    }

    pub fn horizontal(value: Val) -> Self {
        Self {
            left: Some(value),
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn vertical(value: Val) -> Self {
        Self {
            top: Some(value),
            bottom: Some(value),
            ..Default::default()
        }
    }

    pub fn axes(horizontal: Val, vertical: Val) -> Self {
        Self {
            left: Some(horizontal),
            right: Some(horizontal),
            top: Some(vertical),
            bottom: Some(vertical),
        }
    }

    pub fn left(value: Val) -> Self {
        Self {
            left: Some(value),
            ..Default::default()
        }
    }

    pub fn right(value: Val) -> Self {
        Self {
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn top(value: Val) -> Self {
        Self {
            top: Some(value),
            ..Default::default()
        }
    }

    pub fn bottom(value: Val) -> Self {
        Self {
            bottom: Some(value),
            ..Default::default()
        }
    }
}

#[derive(Reflect, Debug, Clone, PartialEq)]
#[reflect(FromReflect)]
pub enum Texture {
    Color(Color),
    Image {
        image: Handle<Image>,
        flip_x: bool,
        flip_y: bool,
        color: Color,
    },
    Atlas {
        atlas: Handle<TextureAtlas>,
        index: usize,
        flip_x: bool,
        flip_y: bool,
        color: Color,
    },
}

impl Default for Texture {
    fn default() -> Self {
        Self::Color(default())
    }
}

impl From<DomAttributeValue> for Option<Color> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => parse_color(&value),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<f64> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Float(value) => Some(value),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<bool> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Bool(value) => Some(value),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<i64> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Int(value) => Some(value),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Texture> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => parse_color(&value).map(Texture::Color),
            DomAttributeValue::Any(value) => match <dyn Reflect>::downcast::<Texture>(value) {
                Ok(value) => Some(*value),
                Err(_) => None,
            },
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<f32> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => {
                let value: f32 = value.parse().unwrap_or_default();
                Some(value)
            }
            DomAttributeValue::Int(value) => Some(value as f32),
            DomAttributeValue::Float(value) => Some(value as f32),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Val> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => Some(parse_size_val(&value)),
            DomAttributeValue::Int(value) => Some(Val::Px(value as f32)),
            DomAttributeValue::Float(value) => Some(Val::Px(value as f32)),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Option<f32>> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => {
                let value: Option<f32> = value.parse::<f32>().ok();
                Some(value)
            }
            DomAttributeValue::Int(value) => Some(Some(value as f32)),
            DomAttributeValue::Float(value) => Some(Some(value as f32)),
            _ => None,
        }
    }
}
/*
impl<'a> IntoAttributeValue<'a> for Color {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        let boxed: bumpalo::boxed::Box<'a, dyn AnyValue> =
            unsafe { bumpalo::boxed::Box::from_raw(bump.alloc(self)) };

        AttributeValue::Any(RefCell::new(Some(boxed)))
    }
}*/

impl From<DomAttributeValue> for Option<BorderColor> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => parse_color(&value).map(BorderColor),
            DomAttributeValue::Any(value) => match <dyn Reflect>::downcast::<Color>(value) {
                Ok(value) => Some(BorderColor(*value)),
                Err(value) => match <dyn Reflect>::downcast::<BorderColor>(value) {
                    Ok(value) => Some(*value),
                    Err(_) => None,
                },
            },
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<UiOptionalRect> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => {
                let mut split = value
                    .split_whitespace()
                    .map(parse_size_val)
                    .collect::<Vec<_>>();
                match split.len() {
                    1 => Some(UiOptionalRect::all(split.pop().unwrap())),
                    2 => {
                        let first = split.pop().unwrap();
                        let second = split.pop().unwrap();
                        Some(UiOptionalRect::axes(second, first))
                    }
                    3 => {
                        let first = split.pop().unwrap();
                        let second = split.pop().unwrap();
                        let three = split.pop().unwrap();
                        Some(UiOptionalRect::new(second, second, first, three))
                    }
                    4 => {
                        let first = split.pop().unwrap();
                        let second = split.pop().unwrap();
                        let three = split.pop().unwrap();
                        let four = split.pop().unwrap();
                        Some(UiOptionalRect::new(four, second, first, three))
                    }
                    _ => None,
                }
            }
            DomAttributeValue::Float(value) => Some(UiOptionalRect::all(Val::Px(value as f32))),
            DomAttributeValue::Int(value) => Some(UiOptionalRect::all(Val::Px(value as f32))),
            DomAttributeValue::Any(value) => {
                let Ok(value) = <dyn Reflect>::downcast::<UiOptionalRect>(value) else {
                    return None;
                };
                Some(*value)
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Display> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "flex" => Some(Display::Flex),
                "grid" => Some(Display::Grid),
                "none" => Some(Display::None),
                _ => None,
            },
            DomAttributeValue::Any(value) => {
                <dyn Reflect>::downcast::<Display>(value).ok().map(|n| *n)
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<PositionType> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "relative" => Some(PositionType::Relative),
                "absolute" => Some(PositionType::Absolute),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<PositionType>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<OptionalOverflow> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "visible" | "visible visible" => Some(OptionalOverflow {
                    x: Some(OverflowAxis::Visible),
                    y: Some(OverflowAxis::Visible),
                }),
                "visible clip" | "visible hidden" => Some(OptionalOverflow {
                    x: Some(OverflowAxis::Clip),
                    y: Some(OverflowAxis::Clip),
                }),
                "clip" | "hidden" | "clip clip" | "hidden hidden" => Some(OptionalOverflow {
                    x: Some(OverflowAxis::Clip),
                    y: Some(OverflowAxis::Clip),
                }),
                "clip visible" | "hidden visible" => Some(OptionalOverflow {
                    x: Some(OverflowAxis::Clip),
                    y: Some(OverflowAxis::Clip),
                }),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<OptionalOverflow>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Direction> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "ltr" => Some(Direction::LeftToRight),
                "rtl" => Some(Direction::RightToLeft),
                "inherit" => Some(Direction::Inherit),
                _ => None,
            },
            DomAttributeValue::Any(value) => {
                <dyn Reflect>::downcast::<Direction>(value).ok().map(|n| *n)
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<AlignItems> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "default" => Some(AlignItems::Default),
                "start" => Some(AlignItems::Start),
                "end" => Some(AlignItems::End),
                "flex-start" => Some(AlignItems::FlexStart),
                "flex-end" => Some(AlignItems::FlexEnd),
                "center" => Some(AlignItems::Center),
                "baseline" => Some(AlignItems::Baseline),
                "stretch" => Some(AlignItems::Stretch),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<AlignItems>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<JustifyItems> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "default" => Some(JustifyItems::Default),
                "start" => Some(JustifyItems::Start),
                "end" => Some(JustifyItems::End),
                "center" => Some(JustifyItems::Center),
                "baseline" => Some(JustifyItems::Baseline),
                "stretch" => Some(JustifyItems::Stretch),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<JustifyItems>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<AlignSelf> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "auto" => Some(AlignSelf::Auto),
                "start" => Some(AlignSelf::Start),
                "end" => Some(AlignSelf::End),
                "flex-start" => Some(AlignSelf::FlexStart),
                "flex-end" => Some(AlignSelf::FlexEnd),
                "center" => Some(AlignSelf::Center),
                "baseline" => Some(AlignSelf::Baseline),
                "stretch" => Some(AlignSelf::Stretch),
                _ => None,
            },
            DomAttributeValue::Any(value) => {
                <dyn Reflect>::downcast::<AlignSelf>(value).ok().map(|n| *n)
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<JustifySelf> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "auto" => Some(JustifySelf::Auto),
                "start" => Some(JustifySelf::Start),
                "end" => Some(JustifySelf::End),
                "center" => Some(JustifySelf::Center),
                "baseline" => Some(JustifySelf::Baseline),
                "stretch" => Some(JustifySelf::Stretch),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<JustifySelf>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<AlignContent> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "default" => Some(AlignContent::Default),
                "start" => Some(AlignContent::Start),
                "end" => Some(AlignContent::End),
                "flex-start" => Some(AlignContent::FlexStart),
                "flex-end" => Some(AlignContent::FlexEnd),
                "center" => Some(AlignContent::Center),
                "stretch" => Some(AlignContent::Stretch),
                "space-evenly" => Some(AlignContent::SpaceEvenly),
                "space-between" => Some(AlignContent::SpaceBetween),
                "space-around" => Some(AlignContent::SpaceAround),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<AlignContent>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<JustifyContent> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "default" => Some(JustifyContent::Default),
                "start" => Some(JustifyContent::Start),
                "end" => Some(JustifyContent::End),
                "flex-start" => Some(JustifyContent::FlexStart),
                "flex-end" => Some(JustifyContent::FlexEnd),
                "center" => Some(JustifyContent::Center),
                "space-evenly" => Some(JustifyContent::SpaceEvenly),
                "space-between" => Some(JustifyContent::SpaceBetween),
                "space-around" => Some(JustifyContent::SpaceAround),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<JustifyContent>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<FlexDirection> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "row" => Some(FlexDirection::Row),
                "column" => Some(FlexDirection::Column),
                "row-reverse" => Some(FlexDirection::RowReverse),
                "column-reverse" => Some(FlexDirection::ColumnReverse),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<FlexDirection>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<FlexWrap> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "no-wrap" => Some(FlexWrap::NoWrap),
                "wrap" => Some(FlexWrap::Wrap),
                "wrap-reverse" => Some(FlexWrap::WrapReverse),
                _ => None,
            },
            DomAttributeValue::Any(value) => {
                <dyn Reflect>::downcast::<FlexWrap>(value).ok().map(|n| *n)
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Visibility> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "visible" => Some(Visibility::Visible),
                "hidden" => Some(Visibility::Hidden),
                "inherit" => Some(Visibility::Inherited),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<Visibility>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Reflect, Clone)]
pub struct TextSections(pub Vec<TextSection>);

impl PartialEq for TextSections {
    fn eq(&self, other: &Self) -> bool {
        self.reflect_partial_eq(other).unwrap_or(false)
    }
}

impl From<DomAttributeValue> for Option<TextSections> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => Some(TextSections(vec![TextSection {
                value,
                style: TextStyle {
                    color: Color::BLACK,
                    ..default()
                },
            }])),
            DomAttributeValue::Any(value) => {
                match <dyn Reflect>::downcast::<Vec<TextSection>>(value) {
                    Ok(value) => Some(TextSections(*value)),
                    Err(value) => match <dyn Reflect>::downcast::<TextSections>(value) {
                        Ok(value) => Some(*value),
                        Err(value) => match <dyn Reflect>::downcast::<TextSection>(value) {
                            Ok(value) => Some(TextSections(vec![*value])),
                            Err(_) => None,
                        },
                    },
                }
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<ZIndex> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Int(value) => Some(ZIndex::Global(value as i32)),
            DomAttributeValue::Any(value) => {
                <dyn Reflect>::downcast::<ZIndex>(value).ok().map(|n| *n)
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Transform> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            // DomAttributeValue::Text(value) => ,
            DomAttributeValue::Any(value) => {
                <dyn Reflect>::downcast::<Transform>(value).ok().map(|n| *n)
            }
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Quat> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            // DomAttributeValue::Text(value) => ,
            DomAttributeValue::Float(value) => {
                Some(Quat::from_rotation_z((value as f32).to_radians()))
            }
            DomAttributeValue::Int(value) => {
                Some(Quat::from_rotation_z((value as f32).to_radians()))
            }
            DomAttributeValue::Any(value) => match <dyn Reflect>::downcast::<Quat>(value) {
                Ok(value) => Some(Quat::from_euler(EulerRot::XYZ, value.x, value.y, value.z)),
                Err(value) => match <dyn Reflect>::downcast::<Quat>(value) {
                    Ok(value) => Some(*value),
                    Err(_) => None,
                },
            },
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<Vec3> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Float(value) => Some(Vec3::Z * (value.to_radians() as f32)),
            DomAttributeValue::Int(value) => Some(Vec3::Z * (value as f32).to_radians()),
            // DomAttributeValue::Text(value) => ,
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<Vec3>(value)
                .ok()
                .map(|n: Box<Vec3>| *n),
            _ => None,
        }
    }
}


impl From<DomAttributeValue> for Option<BreakLineOn> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "word-boundary" => Some(BreakLineOn::WordBoundary),
                "any-character" => Some(BreakLineOn::AnyCharacter),
                "no-wrap" => Some(BreakLineOn::NoWrap),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<BreakLineOn>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<TextAlignment> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "left" => Some(TextAlignment::Left),
                "center" => Some(TextAlignment::Center),
                "right" => Some(TextAlignment::Right),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<TextAlignment>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
}
/* impl From<DomAttributeValue> for Option<Origin> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => match value.as_str() {
                "bottom-left" => Some(Origin::BottomLeft),
                "bottom-right" => Some(Origin::BottomRight),
                "center" => Some(Origin::Center),
                "top-left" => Some(Origin::TopLeft),
                "top-right" => Some(Origin::TopRight),
                _ => None,
            },
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<Origin>(value)
                .ok()
                .map(|n| *n),
            _ => None,
        }
    }
} */
