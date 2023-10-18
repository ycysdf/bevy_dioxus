#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub use dioxus_plugin::*;
pub use ecs_fns::*;
pub use element_attrs::*;
pub use element_core::*;
pub use elements::*;
pub use entity_extra_data::AttrIndex;
pub use smallbox::SmallBox;
pub use text_styled_element::*;

pub mod apc;
pub mod components;
pub mod dioxus_ext;
pub mod dioxus_plugin;
pub mod dom_commands;
pub mod dom_template;
pub mod ecs_apc;
pub mod ecs_fns;
mod element_attrs;
mod element_core;
pub mod elements;
pub mod entity_extra_data;
mod smallbox;
pub mod tailwind;
mod text_styled_element;
pub mod vdm_data;
pub mod vdom_main;

pub mod all_attrs {
    pub use crate::element_attrs::*;
    pub use crate::input_attrs::*;
    pub use crate::text_attrs::*;
}

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::prelude::{Component, Ref};
    pub use bevy_mod_picking::prelude::events;
    pub use bevy_mod_picking::prelude::*;
    pub use dioxus::core::Event;
    pub use dioxus::prelude::*;

    pub use dioxus_elements::input;
    pub use dioxus_elements::*;

    pub use crate::dioxus_ext::{use_cmd_sender, UseStateExt};
    pub use crate::ecs_apc::world_call;
    pub use crate::DioxusPlugin;
    pub use crate::{
        CommonAttrs, CommonCompositeAttrs, ElementAttr, ElementAttrUntyped, ElementTypeBase,
        ElementTypeUnTyped,
    };
    pub use crate::{
        HasIndex, InputAttrs, InputCompositeAttrs, TextAttrs, TextCompositeAttrs, ViewAttrs,
        ViewCompositeAttrs,
    };
    pub use dioxus_elements::extensions::*;

    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    pub mod dioxus_elements {

        pub use crate::{input, text, view};

        pub(crate) mod extensions {
            pub trait CommonAttrsExtension<'a>: crate::prelude::HasAttributes<'a> {}
            impl<'a, T: CommonAttrsExtension<'a>> crate::_CommonAttrsExtension<'a> for T {}
            impl<'a, T: CommonAttrsExtension<'a>> crate::_CommonCompositeAttrsExtension<'a> for T {}
/*
            pub trait CommonEvents<'a>: dioxus::prelude::HasAttributes<'a> + Sized {
                fn onclick(
                    self,
                    value: dioxus::prelude::EventHandler<
                        'a,
                        dioxus::core::Event<
                            bevy_mod_picking::events::Pointer<bevy_mod_picking::events::Click>,
                        >,
                    >,
                ) -> Self {
                    let dd = value.callback.into_inner();
                    self.push_attribute(
                        "onclick",
                        None,
                        dioxus::core::AttributeValue::Listener(value.callback),
                        false,
                    )
                }

                //                fn onclick<E: crate::element_core::EventReturn<T>, T>(self, mut _f: impl FnMut(dioxus::core::Event<bevy_mod_picking::events::Pointer<bevy_mod_picking::events::Click>>) -> E + 'a) -> Self {
                //                    self.push_attribute("onclick", None, _cx.listener(move |e: dioxus::core::Event<bevy_mod_picking::events::Pointer<bevy_mod_picking::events::Click>>| {
                //                        _f(e).spawn(_cx);
                //                    }), false)
                //                }
            }
            impl<'a, T: ViewAttrsExtension<'a>> CommonEvents<'a> for T {}*/


            pub use crate::InputAttrsExtension;
            pub use crate::TextAttrsExtension;
            pub use crate::ViewAttrsExtension;
            pub use crate::_CommonAttrsExtension;
            pub use crate::_CommonCompositeAttrsExtension;
            pub use crate::_InputAttrsExtension;
            pub use crate::_TextAttrsExtension;
            pub use crate::_ViewAttrsExtension;
        }

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
