#![allow(non_snake_case)]

use std::rc::Rc;

use crate::{prelude::*, components::use_controlled_state};

#[derive(Props)]
pub struct CheckboxProps<'a> {
    value: Option<bool>,
    #[props(default = false)]
    initial_value: bool,
    #[props(default = false)]
    readonly: bool,
    onchange: Option<EventHandler<'a, bool>>,
    children: Element<'a>,
}

pub fn Checkbox<'a>(cx: Scope<'a, CheckboxProps>) -> Element<'a> {
    let value = cx.props.value;
    let onchange = &cx.props.onchange;
    let is_checked = use_controlled_state(
        cx,
        value,
        value.unwrap_or(cx.props.initial_value),
        move |value: Rc<bool>| {
            if let Some(onchange) = onchange {
                onchange.call(*value);
            }
        },
    );
    let handle_click = |_| {
        if !cx.props.readonly {
            is_checked.set(!**is_checked);
        }
    };
    let text_str = if **is_checked { "X" } else { " " };
    render! {
        view {
            class: "border border-gray-300 bg-white w-5 h-5 justify-center items-center",
            onclick: handle_click,
            // if **is_checked {"X"} else {" "} todo:
            "{text_str}"
        }
    }
}
