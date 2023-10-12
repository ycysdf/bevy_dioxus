#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::schema_props::COMMON_PROPS_COUNT;
use crate::{impl_schema_type_base, ReflectTextSchemaType, SchemaType, SetAttrValueContext};
use crate::{prelude::*, TextSchemaType};
use bevy::ecs::component::ComponentInfo;
use bevy::ecs::world::EntityMut;
use bevy::reflect::Reflect;
use bevy_cosmic_edit::*;
use std::any::TypeId;

pub fn bevy_color_to_cosmic(color: Color) -> CosmicColor {
    cosmic_text::Color::rgba(
        (color.r() * 255.) as u8,
        (color.g() * 255.) as u8,
        (color.b() * 255.) as u8,
        (color.a() * 255.) as u8,
    )
}
use input_props::*;

impl_schema_type_base!(
    #[derive(Reflect, Debug, Clone, Copy)]
    #[reflect(TextSchemaType)]
    input,
    text_value
);

impl SchemaType for input {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        let attrs = AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::BLACK)));
        let _placeholder_attrs = AttrsOwned::new(
            Attrs::new().color(bevy_color_to_cosmic(Color::hex("#e6e6e6").unwrap())),
        );

        world.spawn((CosmicEditUiBundle {
            border_color: Color::DARK_GRAY.into(),
            style: Style {
                border: UiRect::all(Val::Px(1.)),
                width: Val::Px(100.),
                height: Val::Px(22.),
                ..default()
            },
            text_position: CosmicTextPosition::Left { padding: 4 },
            cosmic_attrs: CosmicAttrs(attrs.clone()),
            cosmic_metrics: CosmicMetrics {
                font_size: 18.,
                line_height: 18. * 1.,
                scale_factor: 1.,
            },
            max_lines: CosmicMaxLines(1),
            text_setter: CosmicText::OneStyle(String::from("")),
            mode: CosmicMode::InfiniteLine,
            ..default()
        },))
    }

    fn try_insert_no_reflect_components(
        &self,
        _entity_mut: &mut EntityMut,
        template_world: &World,
        template_entity: Entity,
        _type_registry: AppTypeRegistry,
        _component_info: &ComponentInfo,
    ) -> bool {
        let type_id = ComponentInfo::type_id(_component_info).unwrap();

        match type_id {
            n if n == TypeId::of::<CosmicAttrs>() => {
                _entity_mut.insert(
                    template_world
                        .get::<CosmicAttrs>(template_entity)
                        .cloned()
                        .unwrap(),
                );
            }
            n if n == TypeId::of::<CosmicText>() => {
                _entity_mut.insert(
                    template_world
                        .get::<CosmicText>(template_entity)
                        .cloned()
                        .unwrap(),
                );
            }
            n if n == TypeId::of::<PlaceholderText>() => {
                _entity_mut.insert(
                    template_world
                        .get::<PlaceholderText>(template_entity)
                        .cloned()
                        .unwrap(),
                );
            }
            n if n == TypeId::of::<PlaceholderAttrs>() => {
                _entity_mut.insert(
                    template_world
                        .get::<PlaceholderAttrs>(template_entity)
                        .cloned()
                        .unwrap(),
                );
            }
            _ => return false,
        }
        true
    }
}

impl TextSchemaType for input {
    fn set_font(
        &self,
        entity_ref: &mut EntityMut,
        v: <crate::schema_props::font as SchemaProp>::Value,
    ) {
        let Some(mut attrs) = entity_ref.get_mut::<CosmicAttrs>() else {
            return;
        };
        // todo: CosmicText font set
        //        attrs.0.family_owned
    }

    fn set_font_size(
        &self,
        entity_ref: &mut EntityMut,
        v: <crate::schema_props::font_size as SchemaProp>::Value,
    ) {
        let Some(mut metrics) = entity_ref.get_mut::<CosmicMetrics>() else {
            return;
        };
        metrics.font_size = v;
    }

    fn set_text_color(
        &self,
        entity_ref: &mut EntityMut,
        v: <crate::schema_props::text_color as SchemaProp>::Value,
    ) {
        let Some(mut attrs) = entity_ref.get_mut::<CosmicAttrs>() else {
            return;
        };
        attrs.0.color_opt = Some(bevy_color_to_cosmic(v));
    }

    fn set_text_linebreak(
        &self,
        entity_ref: &mut EntityMut,
        v: <crate::schema_props::text_linebreak as SchemaProp>::Value,
    ) {
        // todo: CosmicText text_linebreak
    }

    fn set_text_align(
        &self,
        entity_ref: &mut EntityMut,
        v: <crate::schema_props::text_align as SchemaProp>::Value,
    ) {
        let Some(mut pos) = entity_ref.get_mut::<CosmicTextPosition>() else {
            return;
        };
        match v {
            TextAlignment::Left => {
                if !matches!(*pos, CosmicTextPosition::Left { .. }) {
                    *pos = CosmicTextPosition::Left { padding: 0 }
                }
            }
            TextAlignment::Center => {
                *pos = CosmicTextPosition::Center;
            }
            TextAlignment::Right => {
                //                if !matches!(*pos, CosmicTextPosition::Right { .. }) {
                //                    *pos = CosmicTextPosition::Right { padding: 0 }
                //                }
            }
        }
    }
}

pub mod input_props {
    use super::*;

    pub struct text_value;

    impl SchemaProp for text_value {
        type Value = String;

        const TAG_NAME: &'static str = stringify!(value);
        const INDEX: u8 = COMMON_PROPS_COUNT + 0;

        fn set_value(&self, context: &mut SetAttrValueContext, p_value: impl Into<Self::Value>) {
            if let Some(mut t) = context.entity_ref.get_mut::<CosmicText>() {
                *t = CosmicText::OneStyle(p_value.into());
            } else {
                warn!("no found CosmicText component!");
            }
        }
    }
}
