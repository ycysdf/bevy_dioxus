use bevy::prelude::{Deref, DerefMut, Resource, World};
use dioxus::core::ScopeState;

use crate::apc::{self,ApcReceiver, ApcSender};

#[derive(Resource, Deref, DerefMut)]
pub struct EcsApcReceiver(pub ApcReceiver<&'static World>);

#[derive(Clone, Deref)]
pub struct EcsApcSender(pub ApcSender<&'static World>);

pub fn world_call<R: Send + 'static>(cx: &ScopeState, f: impl FnOnce(&'static World) -> R + Send + 'static) -> Box<R> {
    let ecs_sender = cx.consume_context::<EcsApcSender>().unwrap();
    apc::call_with_return(&ecs_sender.0, f)
}
/* 
#[inline]
pub fn use_world_state<T: Send + 'static>(
    cx: &ScopeState,
    f: impl for<'a> FnOnce(&'a World) -> T + Send + 'static,
) -> &UseState<Box<T>> {
    use_state(cx, || world_call(cx, f))
} */
