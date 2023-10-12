#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::any::TypeId;

use crate::prelude::{warn, Entity, TextAlignment, World};
use crate::schema_props::COMMON_PROPS_COUNT;
use crate::{
    impl_schema_type_base, SchemaType, SetAttrValueContext, TextSchemaType,
    TextSections, ReflectTextSchemaType,
};
use bevy::ecs::component::ComponentInfo;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{AppTypeRegistry, Color, TextBundle};
use bevy::reflect::Reflect;
use bevy::text::{BreakLineOn, Text, TextLayoutInfo};
use bevy::ui::widget::TextFlags;
use text_props::*;
impl_schema_type_base!(
    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(TextSchemaType)]
    text,
    sections
);

impl SchemaType for text {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        world.spawn(TextBundle::default())
    }

    fn try_insert_no_reflect_components(
        &self,
        _entity_mut: &mut EntityMut,
        _template_world: &World,
        _template_entity: Entity,
        _type_registry: AppTypeRegistry,
        _component_info: &ComponentInfo,
    ) -> bool {
        let type_id = ComponentInfo::type_id(_component_info).unwrap();

        match type_id {
            n if n == TypeId::of::<TextLayoutInfo>() => {
                _entity_mut.insert(TextLayoutInfo::default());
            }
            _ => return false,
        }
        true
    }
}

impl TextSchemaType for text {
    fn set_font(
        &self,
        entity_ref: &mut EntityMut,
        value: <crate::schema_props::font as SchemaProp>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        for section in t.sections.iter_mut() {
            section.style.font = value.clone();
        }
    }

    fn set_font_size(
        &self,
        entity_ref: &mut EntityMut,
        value: <crate::schema_props::font_size as SchemaProp>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        for section in t.sections.iter_mut() {
            section.style.font_size = value;
        }
    }

    fn set_text_color(
        &self,
        entity_ref: &mut EntityMut,
        value: <crate::schema_props::text_color as SchemaProp>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        for section in t.sections.iter_mut() {
            section.style.color = value;
        }
    }

    fn set_text_linebreak(
        &self,
        entity_ref: &mut EntityMut,
        value: <crate::schema_props::text_linebreak as SchemaProp>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.linebreak_behavior = value;
    }

    fn set_text_align(
        &self,
        entity_ref: &mut EntityMut,
        value: <crate::schema_props::text_align as SchemaProp>::Value,
    ) {
        let Some(mut t) = entity_ref.get_mut::<Text>() else {
            return;
        };
        t.alignment = value;
    }
}

pub mod text_props {
    use super::*;
    pub struct sections;

    impl SchemaProp for sections {
        type Value = TextSections;

        const TAG_NAME: &'static str = "sections";

        const INDEX: u8 = COMMON_PROPS_COUNT + 0;
        fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
            if let Some(mut t) = context.entity_ref.get_mut::<Text>() {
                t.sections = value.into().0;
                if !context.entity_ref.contains::<TextFlags>() {
                    context.entity_ref.insert(TextFlags::default());
                }
                if !context.entity_ref.contains::<TextLayoutInfo>() {
                    context.entity_ref.insert(TextLayoutInfo::default());
                }
            } else {
                context.entity_ref.insert((
                    Text::from_sections(value.into().0),
                    TextFlags::default(),
                    TextLayoutInfo::default(),
                ));
            }
        }
    }
}
