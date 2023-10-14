#![allow(non_snake_case)]

use std::rc::Rc;

use crate::{components::use_controlled_state, prelude::*};

#[derive(Clone)]
pub struct SelectableListContext<T: PartialEq + Clone + Default + 'static> {
    pub selected: UseState<T>,
}

#[derive(Props)]
pub struct SelectableListProps<'a, T: Default + 'static> {
    value: Option<T>,
    #[props(default = default())]
    initial_value: T,
    #[props(default = false)]
    readonly: bool,
    onchange: Option<EventHandler<'a, T>>,
    children: Element<'a>,
}

pub fn SelectableList<'a, T: PartialEq + Clone + Default + 'static>(
    cx: Scope<'a, SelectableListProps<'a, T>>,
) -> Element<'a> {
    let value = &cx.props.value;
    let onchange = &cx.props.onchange;
    let selected = use_controlled_state::<T>(
        cx,
        value.to_owned(),
        value
            .to_owned()
            .unwrap_or_else(|| cx.props.initial_value.to_owned()),
        move |value: Rc<T>| {
            if let Some(onchange) = onchange {
                onchange.call((*value).clone());
            }
        },
    );
    use_shared_state_provider(cx, || SelectableListContext::<T> {
        selected: selected.clone()
    });
    render! {
       view {
           &cx.props.children
       }
    }
}

#[derive(Props)]
pub struct SelectableItemProps<'a, T: PartialEq + Clone + Default + 'static> {
    value: T,
    #[props(default = false)]
    readonly: bool,
    onselected: Option<EventHandler<'a, ()>>,
    children: Element<'a>,
}

pub fn SelectableItem<'a, T: PartialEq + Clone + Default + 'static>(
    cx: Scope<'a, SelectableItemProps<'a, T>>,
) -> Element<'a> {
    let context_state = use_shared_state::<SelectableListContext<T>>(cx).unwrap();
    let is_selected = {
        let context = context_state.read();
        &*context.selected.current() == &cx.props.value
    };
    let handle_click = {
        to_owned!(context_state);
        to_owned!(cx.props.value);
        to_owned!(cx.props.readonly);
        let onselected = &cx.props.onselected;
        move |_| {
            if readonly {
                return;
            }
            {
                let context_state = context_state.read();
                context_state.selected.set(value.clone());
            }
            if let Some(onselected) = onselected {
                onselected.call(());
            }
            context_state.notify_consumers();
        }
    };
    let selected_class = if is_selected {
        "bg-blue-300"
    } else {
        "hover:bg-gray-200"
    };
    render! {
       view {
          onclick: handle_click,
          class: "py-2 px-4 {selected_class}",
          &cx.props.children
       }
    }
}
