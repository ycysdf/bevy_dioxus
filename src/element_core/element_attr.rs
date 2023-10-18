use crate::dom_commands::DomAttributeValue;
use crate::element_attrs::SetAttrValueContext;
use crate::element_core::AttrValue;
use crate::entity_extra_data::AttrIndex;
use crate::smallbox::S1;
use crate::DioxusAttributeDescription;
use crate::SmallBox;

pub trait HasIndex {
    const INDEX: AttrIndex;
}

pub trait ElementAttrUntyped: Send + Sync {
    fn attr_name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;
    fn volatile(&self) -> bool;

    fn attribute_description(&self) -> DioxusAttributeDescription;

    fn index(&self) -> u8;

    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue);
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>);

    fn set_to_default_value(&self, context: &mut SetAttrValueContext);

    fn set_dyn_value_in_class(
        &self,
        context: &mut SetAttrValueContext,
        value: SmallBox<dyn AttrValue, S1>,
    );
}

impl<T: ElementAttr> ElementAttrUntyped for T
where
    Option<T::Value>: From<DomAttributeValue>,
{
    #[inline]
    fn attr_name(&self) -> &'static str {
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
    fn index(&self) -> u8 {
        <T as HasIndex>::INDEX
    }

    #[inline]
    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue) {
        self.set_by_attr_value(context, value)
    }

    #[inline]
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>) {
        self.set_dyn_value(context, value);
    }

    #[inline]
    fn set_to_default_value(&self, context: &mut SetAttrValueContext) {
        self.set_to_default_value(context)
    }

    #[inline]
    fn set_dyn_value_in_class(
        &self,
        context: &mut SetAttrValueContext,
        value: SmallBox<dyn AttrValue, S1>,
    ) {
        self.set_dyn_value_in_class(context, value);
    }
}

pub trait ElementAttr: HasIndex + Send + Sync
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

    #[inline]
    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue) {
        let Some(value) = Into::<Option<Self::Value>>::into(value) else {
            return;
        };
        self.set_value(context, value);
    }

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>);

    #[inline]
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn AttrValue, S1>) {
        if let Ok(value) = value.downcast::<Self::Value>() {
            self.set_value(context, value.into_inner());
        }
    }

    #[inline]
    fn set_to_default_value(&self, context: &mut SetAttrValueContext) {
        self.set_value(context, Self::Value::default_value());
    }

    #[inline]
    fn set_dyn_value_in_class(
        &self,
        context: &mut SetAttrValueContext,
        value: SmallBox<dyn AttrValue, S1>,
    ) {
        if let Ok(value) = value.downcast::<Self::Value>() {
            self.set_value_in_class(context, value.into_inner());
        }
    }

    #[inline]
    fn set_value_in_class(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        let entity_extra_data = context.entity_extra_data();
        if !entity_extra_data.is_set_attr(Self::INDEX) {
            entity_extra_data.set_class_attr(Self::INDEX, true);
            self.set_value(context, value);
        }
    }
}

pub enum SetAttrValueError {}
