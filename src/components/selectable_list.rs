use std::rc::Rc;

#[allow(non_snake_case)]
use crate::{components::use_controlled_state, prelude::*};

#[derive(Clone)]
pub struct SelectableListContext<T: PartialEq + Clone + Default + 'static> {
    pub selected: T,
    pub setter: Rc<dyn Fn(T)>,
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

pub fn SelectableList<'a, T: PartialEq + Clone + Default + std::fmt::Debug + 'static>(
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
    cx.provide_context(SelectableListContext::<T> {
        selected: selected.get().to_owned(),
        setter: selected.setter(),
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
    let update = cx.schedule_update();
    let parent_context = cx.consume_context::<SelectableListContext<T>>().unwrap();
    let is_selected = parent_context.selected == cx.props.value;
    let handle_click = {
        to_owned!(parent_context.setter);
        to_owned!(cx.props.value);
        to_owned!(cx.props.readonly);
        let onselected = &cx.props.onselected;
        move |_| {
            if readonly {
                return;
            }
            (setter)(value.clone());
            (update)();
            if let Some(onselected) = onselected {
                onselected.call(());
            }
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
