#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::NodeBundle;
use crate::{impl_schema_type_base, SchemaType};
use crate::prelude::World;
use bevy_cosmic_edit::*;
use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow},
};

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
        let attrs =
            AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::hex("4d4d4d").unwrap())));
        let placeholder_attrs =
            AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::hex("#e6e6e6").unwrap())));

        world.spawn(CosmicEditUiBundle {
            style: Style {
                width: Val::Px(200.),
                height: Val::Px(50.),
                ..default()
            },
            cosmic_attrs: CosmicAttrs(attrs.clone()),
            cosmic_metrics: CosmicMetrics {
                font_size: 18.,
                line_height: 18. * 1.2,
                scale_factor: 1.0,
            },
            max_lines: CosmicMaxLines(1),
            text_setter: CosmicText::OneStyle(String::default()),
            text_position: CosmicTextPosition::Left { padding: 20 },
            mode: CosmicMode::InfiniteLine,
            placeholder_setter: PlaceholderText(CosmicText::OneStyle("Type something...".into())),
            placeholder_attrs: PlaceholderAttrs(placeholder_attrs.clone()),
            ..default()
        })
    }
}