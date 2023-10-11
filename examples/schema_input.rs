#![allow(non_snake_case)]

use bevy_dioxus::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        DioxusPlugin::new(Root),
        WorldInspectorPlugin::new(),
    ))
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    {
        use bevy::prelude::*;
        use bevy_cosmic_edit::*;
        let entity = commands
            .spawn(CosmicEditUiBundle {
                style: Style {
                    width: Val::Px(1.),
                    height: Val::Px(1.),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            })
            .id();
        commands.insert_resource(Focus(Some(entity)));
    }
}

pub fn Root(cx: Scope) -> Element {
    render! {
        view {
            class: "p-4 flex-col mt-2 gap-2 text-red",
            input{}
        }
    }
}
