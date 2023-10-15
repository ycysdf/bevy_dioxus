use std::ops::Deref;

use bevy::asset::Asset;
use bevy::prelude::Handle;
use bevy::reflect::{FromReflect, TypePath};
use bevy::ui::Val;

use crate::prelude::Reflect;
use crate::smallbox;
use crate::smallbox::S1;
use crate::SmallBox;

pub trait AttrValue: Reflect + Send + Sync + 'static // where Option<Self>: From<DomAttributeValue>
{
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1>;
    fn default_value() -> Self
    where
        Self: Sized;
    fn merge_value(&mut self, _value: Self) where Self:Sized {}
}

impl Clone for SmallBox<dyn AttrValue, S1> {
    fn clone(&self) -> Self {
        self.deref().clone_att_value()
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

#[macro_export]
macro_rules! impl_default_attr_value {
    ($type:ty) => {
        impl AttrValue for $type {
            fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
                smallbox!(self.clone())
            }
            fn default_value() -> Self {
                <Self as Default>::default()
            }
        }
    };
    ($type:ty,$value:stmt) => {
        impl AttrValue for $type {
            fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
                smallbox!(self.clone())
            }
            fn default_value() -> Self {
                $value
            }
        }
    };
}
impl_default_attr_value!(u8);
impl_default_attr_value!(u16);
impl_default_attr_value!(u32);
impl_default_attr_value!(u64);
impl_default_attr_value!(i8);
impl_default_attr_value!(i16);
impl_default_attr_value!(i32);
impl_default_attr_value!(i64);
impl_default_attr_value!(f32);
impl_default_attr_value!(f64);
impl_default_attr_value!(());
impl_default_attr_value!(bool);
impl_default_attr_value!(String);
impl_default_attr_value!(Val);

impl<T: AttrValue + TypePath + FromReflect + Clone> AttrValue for Option<T> {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(match self {
            None => None,
            Some(n) => {
                Some(T::clone(n))
            }
        })
    }

    fn default_value() -> Self
    where
        Self: Sized,
    {
        <Self as Default>::default()
    }
}

impl<T: Asset> AttrValue for Handle<T> {
    fn clone_att_value(&self) -> SmallBox<dyn AttrValue, S1> {
        smallbox!(self.clone())
    }

    fn default_value() -> Self
    where
        Self: Sized,
    {
        <Self as Default>::default()
    }
}
