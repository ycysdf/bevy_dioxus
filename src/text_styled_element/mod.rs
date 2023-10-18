use bevy::ecs::world::EntityMut;

use crate::{element_attrs, get_element_type, SetAttrValueContext};
use crate::prelude::*;

impl<'w, 'e> SetAttrValueContext<'w, 'e> {
    pub fn get_text_styled_element_type(&mut self) -> Option<&'static dyn TextStyledElementType> {
        self.get_text_styled_element_type_by_entity(self.entity_ref.id())
    }

    pub fn get_text_styled_element_type_by_entity(
        &mut self,
        entity: Entity,
    ) -> Option<&'static dyn TextStyledElementType> {
        let schema_name = self
            .entities_extra_data
            .get(&entity)
            .map(|n| n.schema_name)?;
        let schema_type = get_element_type(schema_name);
        let type_registry = self.type_registry.read();
        type_registry
            .get_type_data::<ReflectTextStyledElementType>(schema_type.type_id())
            .and_then(|n| n.get(schema_type.as_reflect()))
    }
}

#[reflect_trait]
pub trait TextStyledElementType {
    fn set_font(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::font as ElementAttr>::Value,
    );
    fn set_font_size(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::font_size as ElementAttr>::Value,
    );
    fn set_text_color(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::text_color as ElementAttr>::Value,
    );
    fn set_text_linebreak(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::text_linebreak as ElementAttr>::Value,
    );
    fn set_text_align(
        &self,
        entity_ref: &mut EntityMut,
        value: <element_attrs::text_align as ElementAttr>::Value,
    );
}

pub fn context_children_scope(
    context: &mut SetAttrValueContext,
    mut f: impl FnMut(Entity, &mut SetAttrValueContext),
) {
    let Some(children) = context.entity_ref.get_mut::<Children>() else {
        return;
    };
    let children: Vec<Entity> = children.into_iter().copied().collect();
    for entity in children {
        f(entity, context);
    }
}

pub fn set_text_value(
    context: &mut SetAttrValueContext,
    mut f: impl FnMut(&'static dyn TextStyledElementType, &mut EntityMut),
) {
    if let Some(text_element_type) = context.get_text_styled_element_type() {
        f(text_element_type, context.entity_ref);
    } else {
        context_children_scope(context, move |entity, context| {
            let Some(text_element_type) = context.get_text_styled_element_type_by_entity(entity)
                else {
                    return;
                };
            context.entity_mut_scope(entity, |entity_ref| {
                f(text_element_type, entity_ref);
            });
        });
    }
}
