/*
use std::rc::Rc;

use dioxus::core::IntoDynNode;

use crate::{
    components::{use_controlled_state, use_selectable_list, SelectableItem, SelectableList},
    prelude::*,
};

pub struct OptionItem<'a, T> {
    pub label: &'a str,
    pub value: T,
}

impl<'a, T: PartialEq + Clone + Default + 'static> DowndownComponent<'a, T>
    for &'a [OptionItem<'a, T>]
{
    fn label(&self, cx: Scope<'a, DropdownProps<'a, T, Self>>) -> Element<'a> {
        let current = cx
            .props
            .value
            .as_ref()
            .and_then(|v| self.iter().find(|n| &n.value == v));
        let label = if let Some(current) = current {
            current.label
        } else {
            "no select"
        };
        render! {
            "{label}"
        }
    }
    fn items(&self, cx: Scope<'a, DropdownProps<'a, T, Self>>) -> Element<'a> {
        render! {
            for n in self{
                SelectableItem::<T> {
                    value: n.value.clone(),
                    "{n.label}"
                }
            }
        }
    }
}

pub trait DowndownComponent<'a, T: PartialEq + Clone + Default + 'static>: Sized {
    fn label(&self, cx: Scope<'a, DropdownProps<'a, T, Self>>) -> Element<'a>;
    fn items(&self, cx: Scope<'a, DropdownProps<'a, T, Self>>) -> Element<'a>;
}

#[derive(Props)]
pub struct DropdownProps<
    'a,
    T: PartialEq + Clone + Default + 'static,
    Items: DowndownComponent<'a, T>,
> {
    #[props(default = "")]
    class: &'a str,
    value: Option<T>,
    #[props(default = default())]
    initial_value: T,
    #[props(default = false)]
    readonly: bool,
    onchange: Option<EventHandler<'a, T>>,
    items: Items,
    children: Element<'a>,
    #[props(extends = ViewAttrs)]
    attributes: Vec<Attribute<'a>>,
}
pub struct SelectableListState<'a, T: PartialEq + Clone + Default + 'static> {
    pub opened: &'a UseState<bool>,
    pub selected: &'a UseState<T>,
}

#[component]
pub fn Dropdown<'a, T: PartialEq + Clone + Default + 'static, Items: DowndownComponent<'a, T>>(
    cx: Scope<'a, DropdownProps<'a, T, Items>>,
) -> Element<'a> {
    let value = &cx.props.value;
    let onchange = &cx.props.onchange;

    let opened = use_state(cx, || false);

    let _selected = use_selectable_list(
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
    let handle_click = |_| {
        if !cx.props.readonly {
            opened.set(true);
        }
    };

    render! {
        view {
            class: "h-10 min-w-30 border border-gray-300 hover:border-black {cx.props.class}",
            ..cx.props.attributes,
            onclick:handle_click,
            cx.props.items.label(cx)
        }
        if *opened.get() {
            cx.props.items.items(cx)
        }
    }
}
*/
