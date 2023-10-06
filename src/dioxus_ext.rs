#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use dioxus::prelude::{ScopeState, UseState};

use crate::EcsSender;

pub trait UseStateExt<T: 'static> {
    fn set_if_changed(&self, value: T);
}

impl<T: PartialEq + 'static> UseStateExt<T> for UseState<T> {
    fn set_if_changed(&self, value: T) {
        if *self.get() != value {
            self.set(value);
        }
    }
}

pub fn use_cmd_sender(cx: &ScopeState) -> EcsSender {
    cx.consume_context::<EcsSender>().unwrap()
}
