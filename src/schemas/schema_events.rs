use std::{ops::Deref, rc::Rc};

use crate::apc::{self};
use bevy::prelude::{Res, World};
use bevy_mod_picking::prelude::{EntityEvent, ListenerInput, On, Pointer};
use dioxus::{
    core::{prelude::EventHandler, ElementId},
    prelude::Event,
};

use crate::vdm_data::VDomData;

pub type PointerEventHandler<'a, T> = EventHandler<'a, Event<Pointer<T>>>;

pub trait DomEvent: EntityEvent + Clone {
    fn dom_event_name() -> &'static str;
}

#[doc(hidden)]
pub trait EventReturn<P>: Sized {
    fn spawn(self, _cx: &dioxus::core::ScopeState) {}
}

impl EventReturn<()> for () {}

#[doc(hidden)]
pub struct AsyncMarker;

impl<T> EventReturn<AsyncMarker> for T
where
    T: std::future::Future<Output = ()> + 'static,
{
    #[inline]
    fn spawn(self, cx: &dioxus::core::ScopeState) {
        cx.spawn(self);
    }
}

pub fn listen_dom_event<T: DomEvent>(world: &mut World, element_id: ElementId) {
    let vdom_data = world.resource::<VDomData>();
    let entity = vdom_data.element_id_to_entity[&element_id];
    let mut entity_ref = world.entity_mut(entity);
    entity_ref.insert(On::<T>::run(
        move |apc_sender: Res<crate::DomApcSender>, event: Res<ListenerInput<T>>| {
            let name = <T as DomEvent>::dom_event_name();
            let data: &T = event.deref();
            let data = data.clone();
            apc::call(&apc_sender.0, move |vdom| {
                vdom.handle_event(&name, Rc::new(data), element_id, false);
            });
        },
    ));
}

pub fn unlisten_dom_event<T: DomEvent>(world: &mut World, element_id: ElementId) {
    let vdom_data: &VDomData = world.resource::<VDomData>();
    let entity = vdom_data.element_id_to_entity[&element_id];
    world.entity_mut(entity).remove::<On<T>>();
}

macro_rules! impl_events {
    (
        $(
            $( #[$attr:meta] )*
            $name:ident: $data:ty
        ),*
    ) => {
        pub fn listen_dom_event_by_name(world: &mut bevy::prelude::World, element_id: dioxus::core::ElementId, name: &str) {
            use bevy_mod_picking::events::*;
            use crate::schema_events::listen_dom_event;

            match name {
                $(
                a if a == <$data as crate::schema_events::DomEvent>::dom_event_name() => listen_dom_event::<$data>(world, element_id),
                )*
                _ => {}
            }
        }
        pub fn unlisten_dom_event_by_name(world: &mut bevy::prelude::World, element_id: dioxus::core::ElementId, name: &str) {
            use bevy_mod_picking::events::*;
            use crate::schema_events::unlisten_dom_event;

            match name {
                $(
                a if a == <$data as crate::schema_events::DomEvent>::dom_event_name() => unlisten_dom_event::<$data>(world, element_id),
                )*
                _ => {}
            }
        }

        $(
            impl crate::schema_events::DomEvent for $data {
                fn dom_event_name() -> &'static str {
                    &stringify!($name)[2..]
                }
            }

            $( #[$attr] )*
            #[inline]
            pub fn $name<'a, E: crate::schemas::schema_events::EventReturn<T>, T>(_cx: &'a dioxus::core::ScopeState, mut _f: impl FnMut(dioxus::core::Event<$data>) -> E + 'a) -> dioxus::core::Attribute<'a> {
                dioxus::core::Attribute::new(
                    stringify!($name),
                    _cx.listener(move |e: dioxus::core::Event<$data>| {
                        _f(e).spawn(_cx);
                    }),
                    None,
                    false,
                )
            }
        )*
    };
}
pub mod events {
    use bevy_mod_picking::prelude::events::*;
    use bevy_mod_picking::prelude::Pointer;
    impl_events![
        onmouseover: Pointer<Over>,
        onmouseout: Pointer<Out>,
        onmousedown: Pointer<Down>,
        onmouseup: Pointer<Up>,
        onclick: Pointer<Click>,
        onmousemove: Pointer<Move>,
        ondragstart: Pointer<DragStart>,
        ondrag: Pointer<Drag>,
        ondragend: Pointer<DragEnd>,
        ondragenter: Pointer<DragEnter>,
        ondragover: Pointer<DragOver>,
        ondragleave: Pointer<DragLeave>,
        ondrop: Pointer<Drop>
    ];
}
