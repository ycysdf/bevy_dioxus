use std::any::Any;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bevy::ecs::system::CommandQueue;
use bevy::log::error;
use bevy::prelude::{AppTypeRegistry, default};
use bevy::reflect::{ReflectFromPtr, ReflectFromReflect};
use dioxus::core::{BorrowedAttributeValue, Mutation, Mutations};
use dioxus::prelude::*;
use futures_util::{FutureExt, select};

use crate::{DomApcReceiver, EcsSender};
use crate::apc::ApcReceiver;
use crate::dom_commands::*;
use crate::ecs_apc::EcsApcSender;

pub enum EcsMsg {
    PushCommandQueue(CommandQueue),
}

pub fn vdom_main(
    cmd_sender: EcsSender,
    ecs_apc_sender: EcsApcSender,
    dom_apc_receiver: DomApcReceiver,
    is_dioxus_rendered: Arc<AtomicBool>,
    type_registry: AppTypeRegistry,
    ui: Component,
) {
    let handle_mutations = |mutations: Mutations| {
        is_dioxus_rendered.store(true, Ordering::Relaxed);
        let mut command_queue = CommandQueue::default();
        if !mutations.templates.is_empty() {
            command_queue.push(CreateTemplates {
                templates: mutations.templates.into_iter().map(|n| n.into()).collect(),
                ..default()
            });
        } else if mutations.edits.is_empty() {
            return;
        }
        for edit in mutations.edits {
            match edit {
                Mutation::AppendChildren { id, m } => {
                    command_queue.push(AppendChildren {
                        stack_pop_count: m,
                        id,
                        ..default()
                    });
                }
                Mutation::AssignId { path, id } => {
                    command_queue.push(AssignId {
                        path,
                        id,
                        ..default()
                    });
                }
                Mutation::CreatePlaceholder { id } => {
                    command_queue.push(CreatePlaceholder { id, ..default() });
                }
                Mutation::CreateTextNode { value, id } => {
                    command_queue.push(CreateTextNode {
                        value: value.to_string(),
                        id,
                        ..default()
                    });
                }
                Mutation::HydrateText { path, value, id } => {
                    command_queue.push(HydrateText {
                        path,
                        value: value.to_string(),
                        id,
                        ..default()
                    });
                }
                Mutation::LoadTemplate { name, index, id } => {
                    command_queue.push(LoadTemplate {
                        name: name.to_string(),
                        root_index: index,
                        element_id: id,
                        ..default()
                    });
                }
                Mutation::ReplaceWith { id, m } => {
                    command_queue.push(ReplaceWith {
                        stack_pop_count: m,
                        id,
                        ..default()
                    });
                }
                Mutation::ReplacePlaceholder { path, m } => {
                    command_queue.push(ReplacePlaceholder {
                        stack_pop_count: m,
                        path,
                        ..default()
                    });
                }
                Mutation::InsertAfter { id, m } => {
                    command_queue.push(InsertAfter {
                        stack_pop_count: m,
                        id,
                        ..default()
                    });
                }
                Mutation::InsertBefore { id, m } => {
                    command_queue.push(InsertBefore {
                        stack_pop_count: m,
                        id,
                        ..default()
                    });
                }
                Mutation::SetAttribute {
                    name,
                    value,
                    id,
                    ns: _ns,
                } => {
                    let value = match value {
                        BorrowedAttributeValue::Text(r) => DomAttributeValue::Text(r.to_string()),
                        BorrowedAttributeValue::Float(r) => DomAttributeValue::Float(r),
                        BorrowedAttributeValue::Int(r) => DomAttributeValue::Int(r),
                        BorrowedAttributeValue::Bool(r) => DomAttributeValue::Bool(r),
                        BorrowedAttributeValue::Any(ref r) => {
                            use bevy::ptr::Ptr;
                            let value = r.as_any();
                            let value_type_id = value.type_id();
                            let value = {
                                let type_registry = type_registry.read();
                                let from_ptr = type_registry
                                    .get_type_data::<ReflectFromPtr>(value_type_id)
                                    .unwrap();

                                let ptr = value as *const dyn Any as *const ();

                                let ptr = std::ptr::NonNull::<u8>::new(ptr as *mut u8).unwrap();

                                let reflect_obj = unsafe { from_ptr.as_reflect_ptr(Ptr::new(ptr)) };

                                let from_reflect = type_registry
                                    .get_type_data::<ReflectFromReflect>(value_type_id)
                                    .unwrap();
                                from_reflect.from_reflect(reflect_obj).unwrap()
                            };
                            DomAttributeValue::Any(value)
                        }
                        BorrowedAttributeValue::None => DomAttributeValue::None,
                    };
                    command_queue.push(SetAttribute {
                        name: unsafe { std::mem::transmute::<&str, &'static str>(name) },
                        value,
                        id,
                        ..default()
                    });
                }
                Mutation::SetText { value, id } => {
                    command_queue.push(SetText {
                        value: value.to_string(),
                        id,
                        ..default()
                    });
                }
                Mutation::Remove { id } => {
                    command_queue.push(Remove { id, ..default() });
                }
                Mutation::NewEventListener { name, id } => {
                    command_queue.push(NewEventListener {
                        name: name.to_string(),
                        id,
                        ..default()
                    });
                }
                Mutation::RemoveEventListener { name, id } => {
                    command_queue.push(RemoveEventListener {
                        name: name.to_string(),
                        id,
                        ..default()
                    });
                }
                Mutation::PushRoot { id } => {
                    command_queue.push(PushRoot { id, ..default() });
                }
            }
        }
        cmd_sender
            .send(EcsMsg::PushCommandQueue(command_queue))
            .unwrap();
    };

    let mut vdom = VirtualDom::new(ui);
    vdom.base_scope().provide_context(ecs_apc_sender);
    vdom.base_scope().provide_context(cmd_sender.clone());
    let mutations = vdom.rebuild();
    handle_mutations(mutations);

    #[cfg(all(feature = "hot-reload", debug_assertions))]
        let (hot_reload_tx, hot_reload_rx) = flume::unbounded::<dioxus_hot_reload::HotReloadMsg>();

    #[cfg(all(feature = "hot-reload", debug_assertions))]
    dioxus_hot_reload::connect({
        move |msg| {
            if let Err(err) = hot_reload_tx.send(msg) {
                error!("send err: {:?}", err);
            }
        }
    });

    futures_executor::block_on(async {
        loop {
            #[cfg(all(feature = "hot-reload", debug_assertions))]
                let mut hot_reload_recv = hot_reload_rx.recv_async().fuse();
            #[cfg(not(all(feature = "hot-reload", debug_assertions)))]
                let mut hot_reload_recv = std::future::pending::<()>().fuse();

            select! {
                _ = vdom.wait_for_work().fuse() => {
                },
                _msg = hot_reload_recv => {
                    #[cfg(feature = "hot-reload")]
                        match _msg {
                            Ok(_msg)=>{
                                match _msg {
                                    dioxus_hot_reload::HotReloadMsg::UpdateTemplate(template) => {
                                        vdom.replace_template(template);
                                    }
                                    dioxus_hot_reload::HotReloadMsg::Shutdown => {
                                        std::process::exit(0);
                                    }
                                }
                            }
                            Err(err)=>{
                                error!("hot_reload_rx error! {:?} .",err);
                                vdom.wait_for_work().await;
                            }
                        }
                }
                msg = dom_apc_receiver.recv_async().fuse() => {
                    match msg {
                        Ok(msg) => {
                            ApcReceiver::send_return(msg,unsafe{std::mem::transmute(&mut vdom)});
                        }
                        Err(err) =>{
                            error!("dom_apc_receiver recv error: {:?}",err);
                        }
                    }
                }
            }
            let mutations = vdom.render_immediate();
            handle_mutations(mutations);
        }
    });
}
