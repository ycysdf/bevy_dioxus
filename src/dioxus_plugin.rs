use std::mem;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use bevy::ecs::system::{Command, CommandQueue, SystemBuffer, SystemMeta};
use bevy::prelude::*;
use bevy::ui::widget::TextFlags;
use bevy_cosmic_edit::{CosmicEditPlugin, CosmicText, Focus, ReadOnly};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::prelude::PickingInteraction;
use dioxus::core::ElementId;
use dioxus::prelude::*;

use crate::{SetAttrValueContext, TextSections, Texture};
use crate::apc::{self};
use crate::ecs_apc::{EcsApcReceiver, EcsApcSender};
use crate::entity_extra_data::EntitiesExtraData;
use crate::prelude::{Click, ListenerInput, On, Pointer};
use crate::tailwind::{handle_interaction_classes, InteractionClass};
use crate::vdm_data::{TemplateData, VDomData};
use crate::vdom_main::{EcsMsg, vdom_main};

#[derive(Component)]
pub struct NodeTemplate;

pub type Component = fn(Scope) -> Element;

pub struct DioxusPlugin {
    ui: Mutex<Option<Component>>,
}

impl DioxusPlugin {
    pub fn new(ui: Component) -> Self {
        Self {
            ui: Mutex::new(Some(ui)),
        }
    }
}

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct EcsReceiver(pub flume::Receiver<EcsMsg>);

#[derive(Deref, Clone, DerefMut)]
pub struct EcsSender(pub flume::Sender<EcsMsg>);

impl EcsSender {
    pub fn send_cmd<C: Command>(&self, cmd: C) {
        let mut queue = CommandQueue::default();
        queue.push(cmd);
        self.send(EcsMsg::PushCommandQueue(queue)).unwrap()
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct DomApcSender(pub apc::ApcSender<&'static mut VirtualDom>);

impl Clone for DomApcSender {
    fn clone(&self) -> Self {
        DomApcSender(apc::ApcSender::clone(&self.0))
    }
}

#[derive(Deref, DerefMut)]
pub struct DomApcReceiver(pub apc::ApcReceiver<&'static mut VirtualDom>);

impl Clone for DomApcReceiver {
    fn clone(&self) -> Self {
        DomApcReceiver(apc::ApcReceiver::clone(&self.0))
    }
}
/*
#[derive(Deref, Clone, DerefMut)]
pub struct EcsEventRegisterSender(pub flume::Sender<()>);

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct EcsEventRegisterReceiver(pub flume::Receiver<()>); */

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct EcsEventSender(pub flume::Sender<()>);

#[derive(Deref, Clone, DerefMut)]
pub struct EcsEventReceiver(pub flume::Receiver<()>);

#[derive(Resource, Default, DerefMut, Deref)]
pub struct TemplateWorld(World);

impl Plugin for DioxusPlugin {
    fn build(&self, app: &mut App) {
        let ui = self.ui.lock().unwrap().take().unwrap();
        let (cmd_sender, cmd_receiver) = flume::unbounded::<EcsMsg>();
        let (ecs_apc_sender, ecs_apc_receiver) = apc::channel();
        let (dom_apc_sender, dom_apc_receiver) = apc::channel();

        let is_dioxus_rendered: Arc<AtomicBool> = Arc::new(false.into());

        let vdom_data = {
            let mut r = VDomData::new();
            r.element_id_to_entity.insert(ElementId(0), {
                let root = app.world.spawn((
                    NodeBundle {
                        visibility: Visibility::Visible,
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("root"),
                ));
                root.id()
            });
            r
        };
        app.add_plugins((DefaultPickingPlugins, CosmicEditPlugin::default()))
            .register_type::<TextFlags>()
            .register_type::<PickingInteraction>()
            .register_type::<InteractionClass>()
            .register_type::<TextSections>()
            .register_type::<Texture>()
            .register_type::<crate::elements::view>()
            .register_type::<crate::elements::input>()
            .register_type::<crate::elements::text>()
            .insert_resource({
                let mut world = World::default();
                world.insert_resource(TemplateData::default());
                world.insert_resource(EntitiesExtraData::default());
                TemplateWorld(world)
            })
            .insert_resource(EntitiesExtraData::default())
            .insert_resource(vdom_data)
            .insert_resource(DomApcSender(dom_apc_sender))
            .insert_resource(EcsReceiver(cmd_receiver))
            .insert_resource(EcsApcReceiver(ecs_apc_receiver))
            .insert_resource(IsDioxusRendered(is_dioxus_rendered.clone()))
            .add_systems(
                Update,
                (
                    focus_input,
                    update_mutations.run_if(|receiver: Res<EcsReceiver>| !receiver.is_empty()),
                    update_interaction_classes,
                    handle_apc.run_if(|rpc_receiver: Res<EcsApcReceiver>| !rpc_receiver.is_empty()),
                ),
            );
        let _handle = std::thread::spawn({
            let type_registry = app.world.resource::<AppTypeRegistry>().clone();
            move || {
                vdom_main(
                    EcsSender(cmd_sender),
                    EcsApcSender(ecs_apc_sender),
                    DomApcReceiver(dom_apc_receiver),
                    is_dioxus_rendered,
                    type_registry,
                    ui,
                );
            }
        });
    }
}

#[derive(Resource, Deref)]
pub struct IsDioxusRendered(Arc<AtomicBool>);

fn handle_apc(
    apc_receiver: Res<EcsApcReceiver>,
    is_dioxus_rendered: Res<IsDioxusRendered>,
    world: &World,
) {
    if apc_receiver.try_recv_all(unsafe { mem::transmute(world) }) {
        loop {
            if is_dioxus_rendered.load(Ordering::Relaxed) {
                is_dioxus_rendered.store(false, Ordering::Relaxed);
                break;
            }

            apc_receiver.try_recv_all(unsafe { mem::transmute(world) });
        }
    }
}

#[derive(Resource, Default)]
pub struct CommandQueues {
    pub queues: Vec<CommandQueue>,
}

impl SystemBuffer for CommandQueues {
    fn apply(&mut self, _system_meta: &SystemMeta, world: &mut World) {
        for mut queue in self.queues.drain(..).into_iter() {
            queue.apply(world);
        }
    }
}

fn update_mutations(vdom_receiver: Res<EcsReceiver>, mut commands: Deferred<CommandQueues>) {
    while let Ok(msg) = vdom_receiver.try_recv() {
        match msg {
            EcsMsg::PushCommandQueue(command_queue) => {
                commands.queues.push(command_queue);
            }
        }
    }
}

pub struct HandleInteractionClassesCommand(Vec<Entity>);

impl Command for HandleInteractionClassesCommand {
    fn apply(self, world: &mut World) {
        let type_registry = world.resource::<AppTypeRegistry>().clone();
        world.resource_scope(|world, mut entities_extra_data: Mut<EntitiesExtraData>| {
            for entity in self.0.into_iter() {
                handle_interaction_classes(&mut SetAttrValueContext {
                    entity_ref: &mut world.entity_mut(entity),
                    entities_extra_data: entities_extra_data.deref_mut(),
                    type_registry: type_registry.clone(),
                });
            }
        });
    }
}

fn update_interaction_classes(
    mut commands: Commands,
    entities: Query<Entity, (With<InteractionClass>, Changed<Interaction>)>,
) {
    if entities.is_empty() {
        return;
    }
    commands.add(HandleInteractionClassesCommand(entities.iter().collect()));
}

fn focus_input(
    inputs: Query<Entity, (Without<ReadOnly>, Added<CosmicText>)>,
    mut commands: Commands,
) {
    for entity in inputs.iter() {
        commands
            .entity(entity)
            .insert(On::<Pointer<Click>>::run(on_click_input));
    }
}

fn on_click_input(listener_input: Res<ListenerInput<Pointer<Click>>>, mut focus: ResMut<Focus>) {
    focus.0 = Some(listener_input.target);
}
