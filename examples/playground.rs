#![allow(non_snake_case)]

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_dioxus::{
    components::{SelectableItem, SelectableList},
    prelude::*,
};

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
}

pub fn Root(cx: Scope) -> Element {
    render! {
       view {
          class: "p-4 flex-col mt-2 gap-2 text-red",
          SelectableList::<u32>{
             SelectableItem::<u32>{
                value: 0,
                onselected: |v|{
                    println!("SELECTED 0");
                },
                "Zero"
             }
             SelectableItem::<u32>{
                 value: 1,
                 onselected: |v|{
                     println!("SELECTED 1");
                 },
                "One"
             }
             SelectableItem::<u32>{
                 value: 2,
                 onselected: |v|{
                     println!("SELECTED 2");
                 },
                "Two"
             }
          }
       }
    }
}
