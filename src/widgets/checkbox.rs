#![allow(non_snake_case)]

use crate::prelude::*;

#[inline_props]
pub fn Checkbox(cx: Scope) -> Element {
    let is_checked = use_state(cx, || false);
    let handle_click = |_| {
        is_checked.set(!**is_checked);
    };
    let text_str = if **is_checked { "X" } else { " " };
    render! {
        view {
            class: "border border-gray-300 bg-white w-10 h-10 justify-center items-center",
            onclick: handle_click,
            // if **is_checked {"X"} else {" "} todo:
            "{text_str}"
        }
    }
}
