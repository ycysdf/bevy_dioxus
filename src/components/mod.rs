use std::rc::Rc;

use crate::prelude::*;
pub use checkbox::*;
pub use selectable_list::*;

mod checkbox;
mod selectable_list;

pub fn use_state_change<'a, T: PartialEq + 'static>(
    cx: &'a ScopeState,
    mut on_change: impl FnMut(Rc<T>),
    state: Rc<T>,
) -> &'a UseState<Rc<T>> {
    let prev_state = use_state(cx, {
        to_owned![state];
        move || state
    });

    if prev_state.get() != &state {
        on_change(state.clone());
        prev_state.set(state);
    }
    prev_state
}

#[inline]
pub fn use_uncontrolled_state<'a, T: PartialEq + 'static>(
    cx: &'a ScopeState,
    initial_state_fn: impl DeferValueOrValue<Value = T>,
    mut on_change: impl FnMut(Rc<T>),
) -> (&'a UseState<T>, &'a UseState<Rc<T>>) {
    let uncontrolled_state = use_state(cx, || initial_state_fn.get());
    let prev_state = use_state_change(cx, on_change, uncontrolled_state.current());

    (uncontrolled_state, prev_state)
}

pub trait DeferValueOrValue<M = ()> {
    type Value;
    fn get(self) -> Self::Value;
}

impl<T: 'static> DeferValueOrValue for T {
    type Value = T;
    fn get(self) -> T {
        self
    }
}
pub struct DeferValue;
impl<T: FnOnce() -> R, R: 'static> DeferValueOrValue<DeferValue> for T {
    type Value = R;
    fn get(self) -> R {
        (self)()
    }
}

pub fn use_controlled_state<'a, T: PartialEq + Clone + 'static>(
    cx: &'a ScopeState,
    value: Option<T>,
    initial_value_fn: impl DeferValueOrValue<Value = T>,
    mut on_change: impl FnMut(Rc<T>),
) -> &'a UseState<T> {
    let (uncontrolled_state, _) = use_uncontrolled_state(cx, initial_value_fn, on_change);

    let value = match value {
        Some(value) => Rc::new(value),
        None => uncontrolled_state.current(),
    };
    let set_state = uncontrolled_state.setter();
    use_state_change(
        cx,
        move |value: Rc<T>| {
            set_state((*value).to_owned());
        },
        value,
    );
    uncontrolled_state
}