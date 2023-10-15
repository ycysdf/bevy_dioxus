use bevy::asset::Asset;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::text::BreakLineOn;
use serde::{Deserialize, Serialize};

use crate::dom_commands::DomAttributeValue;
use crate::element_core::AttrValue;
use crate::entity_extra_data::{EntitiesExtraData, EntityExtraData};
use crate::smallbox::S1;
use crate::tailwind::{parse_color, parse_size_val};
use crate::{
    from_str, get_element_type, impl_default_attr_value, smallbox, ElementTypeUnTyped, MyFromStr,
    ReflectTextStyledElementType, SmallBox, TextStyledElementType,
};
use dioxus::prelude::{AnyValue, IntoAttributeValue};

#[derive(PartialEq, Eq, Debug, Deref, DerefMut, Clone, Copy)]
pub struct Attr<T>(pub T);

impl<'a, T: PartialEq + 'static> IntoAttributeValue<'a> for Attr<T> {
    fn into_value(
        self,
        bump: &'a dioxus::core::exports::bumpalo::Bump,
    ) -> dioxus::core::AttributeValue<'a> {
        let boxed: dioxus::core::exports::bumpalo::boxed::Box<'a, dyn AnyValue> =
            unsafe { dioxus::core::exports::bumpalo::boxed::Box::from_raw(bump.alloc(self.0)) };

        dioxus::core::AttributeValue::Any(std::cell::RefCell::new(Some(boxed)))
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Reflect, Serialize, Deserialize)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct OptionalOverflow {
    pub x: Option<OverflowAxis>,
    pub y: Option<OverflowAxis>,
}

impl MyFromStr for OverflowAxis {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "visible" => Some(OverflowAxis::Visible),
            "clip" => Some(OverflowAxis::Clip),
            _ => None,
        }
    }
}

macro_rules! downcast_chain {
    ($value:ident,$type:ty,$($candidate_type:ty),*) => {
        {
            let r = <dyn Reflect>::downcast::<$type>($value).map(|n| *n);
            $(
                let r = r.or_else(|value| {
                    <dyn Reflect>::downcast::<$candidate_type>(value)
                                .map(|n| *n)
                                .map(Into::into)
                });
                )*
            r.ok()
        }
    };
    ($type:ty) => {
        downcast_chain!($type,);
    }
}

macro_rules! impl_from_attr_value {
    ($type:ty,$($candidate_type:ty),*) => {
        impl From<DomAttributeValue> for Option<$type> {
            fn from(value: DomAttributeValue) -> Self {
                match value {
                    DomAttributeValue::Text(value) => from_str(&value),
                    DomAttributeValue::Any(value) => {
                        downcast_chain!(value,$type,$($candidate_type)*)
                    }
                    _ => None,
                }
            }
        }
    };
    ($type:ty) => {
        impl_from_attr_value!($type,);
    }
}

macro_rules! impl_from_attr_value_only_dyn {
    ($type:ty,$($candidate_type:ty),*) => {
        impl From<DomAttributeValue> for Option<$type> {
            fn from(value: DomAttributeValue) -> Self {
                match value {
                    DomAttributeValue::Any(value) => {
                        downcast_chain!(value,$type,$($candidate_type)*)
                    }
                    _ => None,
                }
            }
        }
    };
    ($type:ty) => {
        impl_from_attr_value_only_dyn!($type,);
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy, Reflect)]
#[reflect(Default, PartialEq)]
pub struct OptionalTransform {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<Vec3>,
}

impl OptionalTransform {
    pub fn is_some(&self) -> [bool; 3] {
        [
            self.translation.is_some(),
            self.rotation.is_some(),
            self.scale.is_some(),
        ]
    }
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
    pub fn values(&self) -> [&Option<Val>; 4] {
        [&self.left, &self.right, &self.top, &self.bottom]
    }
    pub fn zero() -> Self {
        Self {
            left: Some(Val::Px(0.)),
            right: Some(Val::Px(0.)),
            top: Some(Val::Px(0.)),
            bottom: Some(Val::Px(0.)),
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

#[derive(Reflect, Debug, Clone, PartialEq)]
#[reflect(FromReflect)]
pub enum UiTexture {
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

impl MyFromStr for UiTexture {
    fn from_str(s: &str) -> Option<Self> {
        parse_color(s).map(UiTexture::Color)
    }
}

impl MyFromStr for BorderColor {
    fn from_str(s: &str) -> Option<Self> {
        from_str::<Color>(s).map(BorderColor)
    }
}

impl MyFromStr for Color {
    fn from_str(s: &str) -> Option<Self> {
        parse_color(s)
    }
}

impl MyFromStr for UiOptionalRect {
    fn from_str(s: &str) -> Option<Self> {
        let mut split = s.split_whitespace().map(parse_size_val).collect::<Vec<_>>();
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
}

impl MyFromStr for Display {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "flex" => Some(Display::Flex),
            "grid" => Some(Display::Grid),
            "none" => Some(Display::None),
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
            DomAttributeValue::Any(value) => {
                let Ok(value) = <dyn Reflect>::downcast::<Val>(value) else {
                    return None;
                };
                Some(*value)
            }
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

impl From<DomAttributeValue> for Option<UiOptionalRect> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => from_str(&value),
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

impl From<DomAttributeValue> for Option<Quat> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => value
                .parse()
                .map(|value: f32| Quat::from_rotation_z(value.to_radians()))
                .ok(),
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
            DomAttributeValue::Text(value) => value
                .parse()
                .map(|n: f32| Vec3::Z * (n.to_radians() as f32))
                .ok(),
            DomAttributeValue::Float(value) => Some(Vec3::Z * (value.to_radians() as f32)),
            DomAttributeValue::Int(value) => Some(Vec3::Z * (value as f32).to_radians()),
            DomAttributeValue::Any(value) => <dyn Reflect>::downcast::<Vec3>(value)
                .ok()
                .map(|n: Box<Vec3>| *n),
            _ => None,
        }
    }
}

impl<T: Asset> From<DomAttributeValue> for Option<Handle<T>> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Any(value) => {
                <dyn Reflect>::downcast::<Handle<T>>(value).ok().map(|n| *n)
            }
            _ => None,
        }
    }
}

impl MyFromStr for PositionType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "relative" => Some(PositionType::Relative),
            "absolute" => Some(PositionType::Absolute),
            _ => None,
        }
    }
}

impl MyFromStr for OptionalOverflow {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "visible" | "visible visible" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Visible),
                y: Some(OverflowAxis::Visible),
            }),
            "visible clip" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Clip),
                y: Some(OverflowAxis::Clip),
            }),
            "clip" | "clip clip" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Clip),
                y: Some(OverflowAxis::Clip),
            }),
            "clip visible" => Some(OptionalOverflow {
                x: Some(OverflowAxis::Clip),
                y: Some(OverflowAxis::Clip),
            }),
            _ => None,
        }
    }
}

impl MyFromStr for Direction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "ltr" => Some(Direction::LeftToRight),
            "rtl" => Some(Direction::RightToLeft),
            "inherit" => Some(Direction::Inherit),
            _ => None,
        }
    }
}

impl MyFromStr for AlignItems {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(AlignItems::Default),
            "start" => Some(AlignItems::Start),
            "end" => Some(AlignItems::End),
            "flex-start" => Some(AlignItems::FlexStart),
            "flex-end" => Some(AlignItems::FlexEnd),
            "center" => Some(AlignItems::Center),
            "baseline" => Some(AlignItems::Baseline),
            "stretch" => Some(AlignItems::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for JustifyItems {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(JustifyItems::Default),
            "start" => Some(JustifyItems::Start),
            "end" => Some(JustifyItems::End),
            "center" => Some(JustifyItems::Center),
            "baseline" => Some(JustifyItems::Baseline),
            "stretch" => Some(JustifyItems::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for AlignSelf {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(AlignSelf::Auto),
            "start" => Some(AlignSelf::Start),
            "end" => Some(AlignSelf::End),
            "flex-start" => Some(AlignSelf::FlexStart),
            "flex-end" => Some(AlignSelf::FlexEnd),
            "center" => Some(AlignSelf::Center),
            "baseline" => Some(AlignSelf::Baseline),
            "stretch" => Some(AlignSelf::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for JustifySelf {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(JustifySelf::Auto),
            "start" => Some(JustifySelf::Start),
            "end" => Some(JustifySelf::End),
            "center" => Some(JustifySelf::Center),
            "baseline" => Some(JustifySelf::Baseline),
            "stretch" => Some(JustifySelf::Stretch),
            _ => None,
        }
    }
}

impl MyFromStr for AlignContent {
    fn from_str(s: &str) -> Option<Self> {
        match s {
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
        }
    }
}

impl MyFromStr for JustifyContent {
    fn from_str(s: &str) -> Option<Self> {
        match s {
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
        }
    }
}

impl MyFromStr for FlexDirection {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "row" => Some(FlexDirection::Row),
            "column" => Some(FlexDirection::Column),
            "row-reverse" => Some(FlexDirection::RowReverse),
            "column-reverse" => Some(FlexDirection::ColumnReverse),
            _ => None,
        }
    }
}

impl MyFromStr for FlexWrap {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "no-wrap" => Some(FlexWrap::NoWrap),
            "wrap" => Some(FlexWrap::Wrap),
            "wrap-reverse" => Some(FlexWrap::WrapReverse),
            _ => None,
        }
    }
}
impl MyFromStr for Visibility {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "visible" => Some(Visibility::Visible),
            "hidden" => Some(Visibility::Hidden),
            "inherit" => Some(Visibility::Inherited),
            _ => None,
        }
    }
}

impl MyFromStr for BreakLineOn {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "word-boundary" => Some(BreakLineOn::WordBoundary),
            "any-character" => Some(BreakLineOn::AnyCharacter),
            "no-wrap" => Some(BreakLineOn::NoWrap),
            _ => None,
        }
    }
}
impl MyFromStr for TextAlignment {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "left" => Some(TextAlignment::Left),
            "center" => Some(TextAlignment::Center),
            "right" => Some(TextAlignment::Right),
            _ => None,
        }
    }
}

impl AttrValue for UiOptionalRect {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(self.clone())
    }
    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn merge_value(&mut self, value: Self) {
        self.left = self.left.or_else(|| value.left);
        self.right = self.right.or_else(|| value.right);
        self.top = self.top.or_else(|| value.top);
        self.bottom = self.bottom.or_else(|| value.bottom);
    }
}

impl AttrValue for OptionalOverflow {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(self.clone())
    }
    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn merge_value(&mut self, value: Self) {
        self.x = self.x.or_else(|| value.x);
        self.y = self.y.or_else(|| value.y);
    }
}

impl AttrValue for OptionalTransform {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(self.clone())
    }

    fn default_value() -> Self {
        <Self as Default>::default()
    }

    fn merge_value(&mut self, value: Self) {
        self.translation = self.translation.or_else(|| value.translation);
        self.rotation = self.rotation.or_else(|| value.rotation);
        self.scale = self.scale.or_else(|| value.scale);
    }
}

impl_from_attr_value!(OverflowAxis);
impl_from_attr_value!(Color);
impl_from_attr_value!(UiTexture);
impl_from_attr_value!(BorderColor, Color);
impl_from_attr_value!(Display);
impl_from_attr_value!(PositionType);
impl_from_attr_value!(OptionalOverflow);
impl_from_attr_value!(Direction);
impl_from_attr_value!(TextAlignment);
impl_from_attr_value!(AlignItems);
impl_from_attr_value!(JustifyItems);
impl_from_attr_value!(AlignSelf);
impl_from_attr_value!(JustifySelf);
impl_from_attr_value!(AlignContent);
impl_from_attr_value!(JustifyContent);
impl_from_attr_value!(FlexDirection);
impl_from_attr_value!(FlexWrap);
impl_from_attr_value!(Visibility);
impl_from_attr_value!(BreakLineOn);
impl_from_attr_value_only_dyn!(Transform);
impl_from_attr_value_only_dyn!(OptionalTransform);

impl_default_attr_value!(UiTexture, UiTexture::Color(Color::rgba_u8(0, 0, 0, 0)));
impl_default_attr_value!(Color, Color::rgba_u8(0, 0, 0, 0));
impl_default_attr_value!(BorderColor, BorderColor(Color::rgba_u8(0, 0, 0, 0)));

impl_default_attr_value!(BreakLineOn, BreakLineOn::WordBoundary);

impl_default_attr_value!(TextAlignment, TextAlignment::Left);

impl_default_attr_value!(Display);
impl_default_attr_value!(PositionType);
impl_default_attr_value!(Direction);
impl_default_attr_value!(AlignItems);
impl_default_attr_value!(JustifyItems);
impl_default_attr_value!(AlignSelf);
impl_default_attr_value!(JustifySelf);
impl_default_attr_value!(AlignContent);
impl_default_attr_value!(JustifyContent);
impl_default_attr_value!(FlexDirection);
impl_default_attr_value!(FlexWrap);
impl_default_attr_value!(Visibility);
impl_default_attr_value!(TextSections);
impl_default_attr_value!(ZIndex);
impl_default_attr_value!(Transform);
impl_default_attr_value!(Quat);
impl_default_attr_value!(Vec3);
impl_default_attr_value!(OverflowAxis);
