#![allow(unused_parens)]
#![allow(dead_code)]

pub use dioxus_plugin::*;
pub use ecs_fns::*;
pub use schema_core::*;
pub use schemas::*;
pub use smallbox::SmallBox;

pub use crate::schema_props;

pub mod apc;
pub mod dioxus_ext;
pub mod dioxus_plugin;
pub mod dom_commands;
pub mod dom_template;
pub mod ecs_apc;
pub mod ecs_fns;
pub mod entity_extra_data;
mod schema_core;
pub mod schemas;
mod smallbox;
pub mod tailwind;
pub mod vdm_data;
pub mod vdom_main;
pub mod widgets;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::prelude::{Component, Ref};
    pub use bevy_mod_picking::prelude::*;
    pub use dioxus::core::Event;
    pub use dioxus::prelude::*;

    pub use crate::dioxus_ext::{use_cmd_sender, UseStateExt};
    pub use crate::ecs_apc::world_call;
    pub use crate::schemas::schema_events::PointerEventHandler;
    pub use crate::DioxusPlugin;
    pub use crate::{SchemaProp, SchemaPropUntyped, SchemaTypeBase, SchemaTypeUnTyped};
    pub use dioxus_elements::*;

    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    pub mod dioxus_elements {
        pub use crate::schemas::schema_events::*;
        pub use crate::{input, text, view};
    }
}
