use crate::{AttrValue, DioxusAttributeDescription};
use crate::dom_commands::DomAttributeValue;

pub trait ElementCompositeAttrUntyped: Send + Sync {
    fn name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;
    fn volatile(&self) -> bool;

    fn attribute_description(&self) -> DioxusAttributeDescription;
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
}

pub trait ElementCompositeAttr: Send + Sync
    where
        Option<Self::Value>: From<DomAttributeValue>,
{
    type Value: AttrValue + Sized;

    const TAG_NAME: &'static str;
    const NAME: &'static str = Self::TAG_NAME;
    const NAME_SPACE: Option<&'static str> = None;
    const VOLATILE: bool = false;
    const ATTRIBUTE_DESCRIPTION: DioxusAttributeDescription =
        (Self::TAG_NAME, Self::NAME_SPACE, Self::VOLATILE);
}