#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::TextBundle;
use bevy::text::{Text, TextLayoutInfo};
use bevy::ui::widget::TextFlags;

use crate::{impl_schema_type_base, SchemaType, SetAttrValueContext, TextSections};
use crate::prelude::World;
use crate::schema_props::COMMON_PROPS_COUNT;

impl_schema_type_base!(text,sections);

impl SchemaType for text {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        world.spawn(TextBundle::default())
    }
}

pub struct sections;

impl SchemaProp for sections {
    type Value = TextSections;

    const TAG_NAME: &'static str = "sections";

    const INDEX: u8 = COMMON_PROPS_COUNT + 0;
    fn set_value(&self, context: &mut SetAttrValueContext, value: impl Into<Self::Value>) {
        if let Some(mut t) = context.entity_ref.get_mut::<Text>() {
            t.sections = value.into().0;
        } else {
            context.entity_ref.insert((
                Text::from_sections(value.into().0),
                TextFlags::default(),
                TextLayoutInfo::default()
            ));
        }
    }
}

// TextAlignment,BreakLineOn does not implement Default!
/* pub struct alignment;

impl SchemaProp for alignment {
    type Value = TextAlignment;

    const TAG_NAME: &'static str = "alignment";
    const INDEX: u8 = COMMON_PROPS_COUNT + 1;

    fn set_value(&self, entity_ref: &mut EntityMut, value: impl Into<Self::Value>) {
        if let Some(mut t) = entity_ref.get_mut::<Text>() {
            t.alignment = value.into();
        } else {
            warn!("no found Text component!");
        }
    }
} */
/* pub struct linebreak_behavior;

impl SchemaProp for linebreak_behavior {
    type Value = BreakLineOn;

    fn set_value(&self, entity_ref: &mut EntityMut, value: impl Into<Self::Value>) {
        if let Some(mut t) = entity_ref.get_mut::<Text>() {
            t.linebreak_behavior = value.into().0;
        } else {
            warn!("no found Text component!");
        }
    }

    const TAG_NAME: &'static str = "linebreak";
    const INDEX: u8 = COMMON_PROPS_COUNT + 2;
}
 */


