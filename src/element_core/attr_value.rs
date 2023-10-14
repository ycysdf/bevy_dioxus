use std::ops::Deref;

use crate::prelude::Reflect;
use crate::SmallBox;
use crate::smallbox::S1;

pub trait AttrValue: Reflect + Send + Sync + 'static // where Option<Self>: From<DomAttributeValue>
{
    fn clone_prop_value(&self) -> SmallBox<dyn AttrValue, S1>;
    fn default_value() -> Self
        where
            Self: Sized;
    fn merge_value(&mut self, _value: SmallBox<dyn AttrValue, S1>) {}
}

impl Clone for SmallBox<dyn AttrValue, S1> {
    fn clone(&self) -> Self {
        self.deref().clone_prop_value()
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
