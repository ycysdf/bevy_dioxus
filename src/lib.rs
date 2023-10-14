#![allow(unused_parens)]
#![allow(dead_code)]

pub use dioxus_plugin::*;
pub use ecs_fns::*;
pub use element_attrs::*;
pub use element_core::*;
pub use elements::*;
pub use smallbox::SmallBox;

pub mod apc;
pub mod dioxus_ext;
pub mod dioxus_plugin;
pub mod dom_commands;
pub mod dom_template;
pub mod ecs_apc;
pub mod ecs_fns;
pub mod entity_extra_data;
mod element_core;
pub mod elements;
mod smallbox;
pub mod tailwind;
pub mod vdm_data;
pub mod vdom_main;
pub mod components;
mod element_attrs;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::prelude::{Component, Ref};
    pub use bevy_mod_picking::prelude::*;
    pub use bevy_mod_picking::prelude::events;
    pub use dioxus::core::Event;
    pub use dioxus::prelude::*;

    pub use dioxus_elements::*;
    pub use dioxus_elements::input;

    pub use crate::{ElementAttr, ElementAttrUntyped, ElementTypeBase, ElementTypeUnTyped};
    pub use crate::dioxus_ext::{use_cmd_sender, UseStateExt};
    pub use crate::DioxusPlugin;
    pub use crate::ecs_apc::world_call;

    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    pub mod dioxus_elements {
        pub use crate::{input, text, view};

        pub mod events {
            use crate::impl_events;
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
    }
}
