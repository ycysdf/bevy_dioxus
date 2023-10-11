#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::prelude::{Click, ListenerInput, On, Pointer, World};
use crate::{default_clone_component, impl_schema_type_base, PropValue, SchemaType};
use bevy::ecs::component::ComponentInfo;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy_cosmic_edit::*;
use std::any::{Any, TypeId};

fn bevy_color_to_cosmic(color: Color) -> CosmicColor {
    cosmic_text::Color::rgba(
        (color.r() * 255.) as u8,
        (color.g() * 255.) as u8,
        (color.b() * 255.) as u8,
        (color.a() * 255.) as u8,
    )
}
impl_schema_type_base!(input);

impl SchemaType for input {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        let attrs = AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::BLACK)));
        let placeholder_attrs = AttrsOwned::new(
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
