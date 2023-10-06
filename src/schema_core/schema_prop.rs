use crate::{PropValue, SmallBox};
use crate::dom_commands::DomAttributeValue;
use crate::SetAttrValueContext;
use crate::smallbox::S1;

pub type DioxusAttributeDescription = (&'static str, Option<&'static str>, bool);

pub trait SchemaPropUntyped: Send + Sync {
    fn name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;
    fn volatile(&self) -> bool;

    fn attribute_description(&self) -> DioxusAttributeDescription;

    fn index(&self) -> u8;

    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue);
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn PropValue, S1>);

    fn set_to_default_value(&self, context: &mut SetAttrValueContext);

    fn set_dyn_value_in_class(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn PropValue, S1>);
}

impl<T: SchemaProp> SchemaPropUntyped for T
    where Option<T::Value>: From<DomAttributeValue>
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
    fn index(&self) -> u8 {
        T::INDEX
    }

    #[inline]
    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue) {
        self.set_by_attr_value(context, value)
    }

    #[inline]
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn PropValue, S1>) {
        self.set_dyn_value(context, value);
    }

    #[inline]
    fn set_to_default_value(&self, context: &mut SetAttrValueContext) {
        self.set_to_default_value(context)
    }

    #[inline]
    fn set_dyn_value_in_class(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn PropValue, S1>) {
        self.set_dyn_value_in_class(context, value);
    }
}


pub trait SchemaProp: Send + Sync
    where Option<Self::Value>: From<DomAttributeValue>
{
    type Value: /*Reflect + */Default + 'static + Sized;

    const TAG_NAME: &'static str;
    const NAME: &'static str = Self::TAG_NAME;
    const NAME_SPACE: Option<&'static str> = None;
    const VOLATILE: bool = false;
    const ATTRIBUTE_DESCRIPTION: DioxusAttributeDescription = (Self::TAG_NAME, Self::NAME_SPACE, Self::VOLATILE);
    const INDEX: u8;

    #[inline]
    fn set_by_attr_value(&self, context: &mut SetAttrValueContext, value: DomAttributeValue) {
        let Some(value) = Into::<Option<Self::Value>>::into(value) else { return; };
        self.set_value(context, value);
    }

    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>);

    #[inline]
    fn set_dyn_value(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn PropValue, S1>) {
        if let Ok(value) = value.downcast::<Self::Value>() {
            self.set_value(context, value.into_inner());
        }
    }

    #[inline]
    fn set_to_default_value(&self, context: &mut SetAttrValueContext) {
        self.set_value(context, Self::Value::default());
    }

    #[inline]
    fn set_dyn_value_in_class(&self, context: &mut SetAttrValueContext, value: SmallBox<dyn PropValue, S1>) {
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