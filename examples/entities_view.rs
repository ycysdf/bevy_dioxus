#![allow(non_snake_case)]

use bevy_dioxus::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, DioxusPlugin::new(Root)))
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
    // let entity_infos = world_call(cx, |world| {
    // });
    render! {
        WorldView{

        }
    }
}

#[inline_props]
fn WorldView(cx: Scope) -> Element {
    let handle_refresh_click = move |_: Event<Pointer<Click>>| {
        cx.needs_update();
    };
    let cmd_sender = use_cmd_sender(cx);
    let handle_spawn_click = move |_: Event<Pointer<Click>>| {
        cmd_sender.send_cmd(|world: &mut World| {
            world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
                world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
                    info!("SPAWN");
                    world.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        ..default()
                    });
                });
            });
        });
    };

    let entity_infos = world_call(cx, |world| {
        world
            .iter_entities()
            .filter(|n| !n.contains::<Parent>())
            .map(get_entity_info)
    });

    render! {
        view{
            name: "world-view",
            class: "min-w-60 bg-gray-300 p-4 flex-col flex-grow-1 items-stretch gap-2",
            view {
                name: "world-view-toolbar",
                class: "flex-row gap-2",
                view {
                    name: "refresh-btn",
                    class: "p-2 bg-gray-200 hover:bg-gray-100 active:bg-blue-200",
                    onclick: handle_refresh_click,
                    "refresh"
                }
                view {
                    name: "spawn-btn",
                    class: "p-2 bg-gray-200 hover:bg-gray-100 active:bg-blue-200",
                    onclick: handle_spawn_click,
                    "spawn"
                }
            }
            view {
                name: "world-entities",
                class: "flex-col flex-grow-1 items-stretch",
                for entity in entity_infos {
                    EntityItem {
                        data: entity,
                        level: 0
                    }
                }
            }
        }
    }
}

#[inline_props]
fn EntityItemChildren(cx: Scope, entity: Entity, level: u8) -> Element {
    let entity_infos = world_call(cx, {
        to_owned![entity];
        move |world| {
            world.entity(entity).get::<Children>().map(|c| {
                c.into_iter()
                    .copied()
                    .map(|n| world.entity(n))
                    .map(get_entity_info)
            })
        }
    });
    if entity_infos.is_none() {
        return None;
    }
    let entity_infos = entity_infos.unwrap();

    render! {
        view {
            name: "child-item-children",
            class: "flex-col items-stretch",
            for entity in entity_infos {
                EntityItem {
                    data: entity,
                    level: *level
                }
            }
        }
    }
}

#[derive(PartialEq)]
struct EntityInfo {
    id: Entity,
    name: &'static str,
    has_child: bool,
}

fn get_entity_info(entity_ref: EntityRef<'static>) -> EntityInfo {
    EntityInfo {
        id: entity_ref.id(),
        name: entity_ref
            .get::<Name>()
            .map(|n| n.as_str())
            .unwrap_or("No Name"),
        has_child: entity_ref.get::<Children>().map(|n| !n.is_empty()) == Some(true),
    }
}

#[inline_props]
fn EntityItem(cx: Scope, data: EntityInfo, level: u8) -> Element {
    let is_expand = use_state(cx, || false);
    let handle_expand_click = |_| {
        is_expand.set(!**is_expand);
    };
    render! {
        view {
            name: "entity-item",
            class: "p-1 bg-gray-200 hover:bg-gray-100 active:bg-blue-200 items-center",
            for _ in (0..*level) {
                view {
                    name: "entity-item-indicator",
                    class: "w-5 h-5 items-center justify-center",
                    "|"
                }
            }
            view {
                name: "entity-item-expand-btn",
                class: "w-5 h-5 items-center justify-center hover:bg-gray-200",
                visibility: if data.has_child { "visible" } else { "hidden" },
                rotation: if **is_expand { 90 } else { 0 },
                onclick: handle_expand_click,
                ">"
            }
            view {
                name: "entity-item-icon",
                class: "w-5 h-5 items-center justify-center hover:bg-gray-200",
                "E"
            }
            "{data.name} {data.id:?}"
        }
        if **is_expand {
            rsx!(EntityItemChildren {
                entity: data.id,
                level: level+1
            })
        }
    }
}
