use smallvec::SmallVec;

use crate::SmallBox;
use crate::{
    AttrValue, DioxusAttributeDescription, ElementAttrUntyped, SetAttrValueContext,
};
use crate::dom_commands::DomAttributeValue;
use crate::smallbox::S1;

pub trait OptionalValue {
    fn get_count(&self) -> u8 {
        1
    }
    fn get_valid_indices_bits(&self) -> u8 {
        1
    }
}

pub trait ElementCompositeAttrUntyped: Send + Sync {
    fn name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;
    fn volatile(&self) -> bool;

    fn attribute_description(&self) -> DioxusAttributeDescription;
    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue);
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>);

    fn get_attrs(
        &self,
        value: &dyn OptionalValue,
    ) -> SmallVec<[&'static dyn ElementAttrUntyped; 4]>;
    fn set_by_attr_value_and_get_attrs(
        &self,
        context: &mut SetAttrValueContext,
        value: DomAttributeValue,
    ) -> Option<SmallVec<[&'static dyn ElementAttrUntyped; 4]>>;
}

impl<T: ElementCompositeAttr> ElementCompositeAttrUntyped for T
    where
        Option<T::Value>: From<DomAttributeValue>,
{
    #[inline]
    fn name(&self) -> &'static str {
        T::NAME
    }

    #[inline]
    fn namespace(&self) -> Option<&'static str> {
        T::NAME_SPACE
    }

    #[inline]
    fn volatile(&self) -> bool {
        T::VOLATILE
    }

    #[inline]
    fn attribute_description(&self) -> DioxusAttributeDescription {
        T::ATTRIBUTE_DESCRIPTION
    }

    #[inline]
    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue) {
        self.set_by_attr_value(context, value)
    }

    #[inline]
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>) {
        self.set_dyn_value(context, value)
    }

    #[inline]
    fn get_attrs(
        &self,
        value: &dyn OptionalValue,
    ) -> SmallVec<[&'static dyn ElementAttrUntyped; 4]> {
        self.get_attrs(value)
    }

    fn set_by_attr_value_and_get_attrs(
        &self,
        context: &mut SetAttrValueContext,
        value: DomAttributeValue,
    ) -> Option<SmallVec<[&'static dyn ElementAttrUntyped; 4]>> {
        self.set_by_attr_value_and_get_attrs(context, value)
    }
}

pub trait ElementCompositeAttr: Send + Sync
    where
        Option<Self::Value>: From<DomAttributeValue>,
{
    type Value: AttrValue + OptionalValue + Sized;

    const TAG_NAME: &'static str;
    const NAME: &'static str = Self::TAG_NAME;
    const NAME_SPACE: Option<&'static str> = None;
    const VOLATILE: bool = false;
    const ATTRIBUTE_DESCRIPTION: DioxusAttributeDescription =
        (Self::TAG_NAME, Self::NAME_SPACE, Self::VOLATILE);

    const ATTRS: &'static [&'static dyn ElementAttrUntyped];

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>);

    #[inline]
    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue) {
        let Some(value) = Into::<Option<Self::Value>>::into(value) else {
            return;
        };
        self.set_value(context, value);
    }
    #[inline]
    fn set_by_attr_value_and_get_attrs(
        &self,
        context: &mut SetAttrValueContext,
        value: DomAttributeValue,
    ) -> Option<SmallVec<[&'static dyn ElementAttrUntyped; 4]>> {
        let Some(value) = Into::<Option<Self::Value>>::into(value) else {
            return None;
        };
        let attrs = self.get_attrs(&value);
        self.set_value(context, value);
        Some(attrs)
    }

    #[inline]
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>) {
        if let Ok(value) = value.downcast::<Self::Value>() {
            self.set_value(context, value.into_inner());
        }
    }
    fn get_attrs(
        &self,
        value: &dyn OptionalValue,
    ) -> SmallVec<[&'static dyn ElementAttrUntyped; 4]> {
        let valid_indices_bits = value.get_valid_indices_bits();
        (0..value.get_count())
            .into_iter()
            .filter(|i| (i >> valid_indices_bits) & 1 == 1)
            .map(|i| Self::ATTRS[i as usize])
            .collect()
    }

    //    fn get_attr_and_values(
    //        &self,
    //        value: Self::Value,
    //    ) -> SmallVec<[(&'static dyn ElementAttrUntyped, SmallBox<dyn AttrValue, S1>); 4]>;
}
