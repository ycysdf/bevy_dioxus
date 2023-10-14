use std::ops::Deref;

use bevy::prelude::{Component, Visibility};
use bevy::reflect::Reflect;
use bevy::text::BreakLineOn;
use bevy::ui::*;
use bevy::utils::{default, HashMap};
use smallvec::{smallvec, SmallVec};

pub use colors::*;

use crate::{ElementAttrUntyped, ElementTypeUnTyped, element_attrs, SetAttrValueContext};
use crate::{smallbox, SmallBox};
use crate::{OptionalOverflow, Texture, try_get_element_type, UiOptionalRect};
use crate::element_core::AttrValue;
use crate::prelude::{TextAlignment, warn};
use crate::smallbox::S1;

mod colors;

#[derive(Default)]
pub struct TailwindClassItem(
    pub SmallVec<[(&'static dyn ElementAttrUntyped, SmallBox<dyn AttrValue, S1>); 4]>,
    pub Interaction,
);

impl Clone for TailwindClassItem {
    fn clone(&self) -> Self {
        let mut r = SmallVec::with_capacity(self.0.capacity());
        for (prop, value) in self.0.iter() {
            r.push((*prop as _, value.deref().clone_prop_value()));
        }
        TailwindClassItem(r, Clone::clone(&self.1))
    }
}

#[derive(Component, Reflect, Default, Debug, PartialEq)]
pub struct InteractionClass;

pub fn handle_classes(context: &mut SetAttrValueContext, classes: &str) {
    let mut set_bits = 0;

    let interaction = context
        .entity_ref
        .get::<Interaction>()
        .copied()
        .unwrap_or_default();
    let mut interaction_classes = smallvec![];
    let mut items = classes
        .split_whitespace()
        .into_iter()
        .map(parse_class)
        .enumerate()
        .collect::<Vec<_>>();
    items.sort_by(|(index, n), (other_index, other)| {
        use std::cmp::Ordering;
        let left = n.1;
        let right = other.1;
        match left {
            Interaction::Pressed => match right {
                Interaction::Pressed => index.cmp(other_index),
                Interaction::Hovered => Ordering::Greater,
                Interaction::None => Ordering::Greater,
            },
            Interaction::Hovered => match right {
                Interaction::Pressed => Ordering::Less,
                Interaction::Hovered => index.cmp(other_index),
                Interaction::None => Ordering::Greater,
            },
            Interaction::None => match right {
                Interaction::Pressed => Ordering::Less,
                Interaction::Hovered => Ordering::Less,
                Interaction::None => index.cmp(other_index),
            },
        }
    });
    let mut normal_props_map = HashMap::new();
    for (_index, item) in items {
        if item.1 != Interaction::None {
            interaction_classes.push(Clone::clone(&item));
        }

        for (prop, value) in item.0.into_iter() {
            match (item.1, interaction) {
                (Interaction::Pressed, Interaction::Pressed)
                | (Interaction::Hovered, Interaction::Hovered | Interaction::Pressed) => {
                    prop.set_dyn_value_in_class(context, value);
                    set_bits |= (1 << prop.index());
                }
                (Interaction::None, _) => {
                    if let Some(previous_value) = normal_props_map.remove(&prop.index()) {
                        let mut value = value.clone();
                        value.merge_value(previous_value);
                        normal_props_map.insert(prop.index(), value);
                    } else {
                        normal_props_map.insert(prop.index(), value.clone());
                    }
                    prop.set_dyn_value_in_class(context, value);
                    set_bits |= (1 << prop.index());
                }
                _ => {}
            }
        }
    }
    if !interaction_classes.is_empty() {
        context.entity_ref.insert(InteractionClass);
    } else {
        context.entity_ref.remove::<InteractionClass>();
    }
    let entity_extra_data = context.entity_extra_data();
    let schema_type: &dyn ElementTypeUnTyped =
        try_get_element_type(entity_extra_data.schema_name).unwrap();

    entity_extra_data.interaction_classes = interaction_classes;
    entity_extra_data.normal_props_map = normal_props_map;
    for prop_index in entity_extra_data.iter_class_attr_indices_exclude(set_bits) {
        let prop = schema_type.prop_by_index(prop_index);
        prop.set_to_default_value(context);
    }
}

pub fn handle_interaction_classes(context: &mut SetAttrValueContext) {
    let interaction = context
        .entity_ref
        .get::<Interaction>()
        .copied()
        .unwrap_or_default();

    let mut set_bits: u64 = 0;
    let mut unset_bits: u64 = 0;
    let items = Clone::clone(&context.entity_extra_data().interaction_classes);
    for item in items.into_iter() {
        for (prop, value) in item.0.into_iter() {
            match (item.1, interaction) {
                (Interaction::Pressed, Interaction::Pressed)
                | (Interaction::Hovered, Interaction::Hovered | Interaction::Pressed) => {
                    if set_bits & !(1 << prop.index()) == set_bits {
                        prop.set_dyn_value_in_class(context, value);
                        set_bits |= (1 << prop.index());
                    }
                }
                (Interaction::None, _) => {
                    warn!("This is not a interaction class!")
                }
                _ => {
                    unset_bits |= (1 << prop.index());
                }
            }
        }
    }
    let schema_type: &dyn ElementTypeUnTyped =
        try_get_element_type(context.entity_extra_data().schema_name).unwrap();

    let num = unset_bits & (!set_bits) & (!context.entity_extra_data().attr_is_set);
    for prop_index in (0..64).filter(move |i| (num >> i) & 1 == 1) {
        let prop = schema_type.prop_by_index(prop_index);
        if let Some(value) = context
            .entity_extra_data()
            .normal_props_map
            .get(&prop_index)
            .cloned()
        {
            prop.set_dyn_value(context, value);
        } else {
            prop.set_to_default_value(context);
        }
    }
}

pub fn parse_size_val(text: &str) -> Val {
    match text {
        "full" => Val::Percent(1.0),
        "auto" => Val::Auto,
        class => {
            if class.ends_with('%') {
                Val::Percent(
                    class
                        .strip_suffix('%')
                        .unwrap()
                        .parse::<f32>()
                        .unwrap_or(0.0)
                        / 100.0,
                )
            } else if class.ends_with("px") {
                Val::Px(class.parse::<f32>().unwrap_or(0.0))
            } else {
                Val::Px(class.parse::<f32>().unwrap_or(0.0) * 4.0)
            }
        }
    }
}

fn parse_class_inner<'a>(
    class: &'a str,
) -> SmallVec<[(&'static dyn ElementAttrUntyped, SmallBox<dyn AttrValue, S1>); 4]> {
    match class {
        "visible" => smallvec![(
            &element_attrs::visibility as _,
            smallbox!(Visibility::Visible)
        ),],
        "invisible" => smallvec![(
            &element_attrs::visibility as _,
            smallbox!(Visibility::Hidden)
        ),],
        "flex-col" => smallvec![
            (&element_attrs::display as _, smallbox!(Display::Flex)),
            (
                &element_attrs::flex_direction as _,
                smallbox!(FlexDirection::Column)
            )
        ],
        "flex-row" => smallvec![
            (&element_attrs::display as _, smallbox!(Display::Flex)),
            (
                &element_attrs::flex_direction as _,
                smallbox!(FlexDirection::Row)
            )
        ],
        "grid" => smallvec![(&element_attrs::display as _, smallbox!(Display::Grid)),],
        "shrink" => smallvec![(&element_attrs::flex_shrink as _, smallbox!(1.0)),],
        "shrink-0" => smallvec![(&element_attrs::flex_shrink as _, smallbox!(0.0)),],
        "grow" => smallvec![(&element_attrs::flex_grow as _, smallbox!(1.0)),],
        "grow-0" => smallvec![(&element_attrs::flex_grow as _, smallbox!(0.0)),],
        class => {
            if let Some(index) = class.strip_prefix("z-") {
                if let Ok(index) = index.parse() {
                    smallvec![(
                        &element_attrs::z_index as _,
                        smallbox!(ZIndex::Global(index))
                    ),]
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("justify-") {
                smallvec![(
                    &element_attrs::justify_content as _,
                    smallbox!(match class {
                        "start" => JustifyContent::Start,
                        "end" => JustifyContent::End,
                        "center" => JustifyContent::Center,
                        "between" => JustifyContent::SpaceBetween,
                        "around" => JustifyContent::SpaceAround,
                        "evenly" => JustifyContent::SpaceEvenly,
                        _ => {
                            JustifyContent::Default
                        }
                    })
                ),]
            } else if let Some(class) = class.strip_prefix("items-") {
                smallvec![(
                    &element_attrs::align_items as _,
                    smallbox!(match class {
                        "start" => AlignItems::FlexStart,
                        "end" => AlignItems::FlexEnd,
                        "center" => AlignItems::Center,
                        "baseline" => AlignItems::Baseline,
                        "stretch" => AlignItems::Stretch,
                        _ => {
                            AlignItems::Default
                        }
                    })
                ),]
            } else if let Some(class) = class.strip_prefix("gap-") {
                let gap = parse_size_val(class);
                smallvec![
                    (&element_attrs::column_gap as _, smallbox!(gap)),
                    (&element_attrs::row_gap as _, smallbox!(gap)),
                ]
            } else if let Some(class) = class.strip_prefix("gap-x-") {
                let gap = parse_size_val(class);
                smallvec![(&element_attrs::row_gap as _, smallbox!(gap)),]
            } else if let Some(class) = class.strip_prefix("gap-y-") {
                let gap = parse_size_val(class);
                smallvec![(&element_attrs::column_gap as _, smallbox!(gap)),]
            } else if class == "relative" {
                smallvec![(
                    &element_attrs::position_type as _,
                    smallbox!(PositionType::Relative)
                ),]
            } else if class == "absolute" {
                smallvec![(
                    &element_attrs::position_type as _,
                    smallbox!(PositionType::Absolute)
                ),]
            } else if class == "hidden" {
                smallvec![(&element_attrs::display as _, smallbox!(Display::None)),]
            } else if let Some(class) = class.strip_prefix("left-") {
                smallvec![(&element_attrs::left as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("right-") {
                smallvec![(&element_attrs::right as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("top-") {
                smallvec![(&element_attrs::top as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("bottom-") {
                smallvec![(&element_attrs::bottom as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("flex-") {
                if let Some(val) = (match class {
                    "wrap" => Some(FlexWrap::Wrap),
                    "wrap-reverse" => Some(FlexWrap::WrapReverse),
                    "nowrap" => Some(FlexWrap::NoWrap),
                    _ => None,
                }) {
                    smallvec![(&element_attrs::flex_wrap as _, smallbox!(val))]
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("w-") {
                smallvec![(&element_attrs::width as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("h-") {
                smallvec![(&element_attrs::height as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("min-w-") {
                smallvec![(
                    &element_attrs::min_width as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("min-h-") {
                smallvec![(
                    &element_attrs::min_height as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("max-w-") {
                smallvec![(
                    &element_attrs::min_width as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("max-h-") {
                smallvec![(
                    &element_attrs::max_height as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("bg-") {
                if let Some(color) = parse_color(class) {
                    smallvec![(
                        &element_attrs::background as _,
                        smallbox!(Texture::Color(color))
                    ),]
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("text-") {
                match class {
                    "nowrap" => smallvec![(
                        &element_attrs::text_linebreak as _,
                        smallbox!(BreakLineOn::NoWrap)
                    ),],
                    "left" => smallvec![(
                        &element_attrs::text_align as _,
                        smallbox!(TextAlignment::Left)
                    ),],
                    "center" => smallvec![(
                        &element_attrs::text_align as _,
                        smallbox!(TextAlignment::Center)
                    ),],
                    "right" => smallvec![(
                        &element_attrs::text_align as _,
                        smallbox!(TextAlignment::Right)
                    ),],
                    _ => {
                        if let Some(color) = parse_color(class) {
                            smallvec![(&element_attrs::text_color as _, smallbox!(color)),]
                        } else if let Ok(size) = class.parse::<f32>() {
                            smallvec![(&element_attrs::font_size as _, smallbox!(size)),]
                        } else {
                            default()
                        }
                    }
                }
            } else if let Some(class) = class.strip_prefix("p-") {
                let padding = parse_size_val(class);
                smallvec![(
                    &element_attrs::padding as _,
                    smallbox!(UiOptionalRect::all(padding))
                ),]
            } else if let Some(class) = class.strip_prefix("py-") {
                smallvec![(
                    &element_attrs::padding as _,
                    smallbox!(UiOptionalRect::vertical(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("px-") {
                smallvec![(
                    &element_attrs::padding as _,
                    smallbox!(UiOptionalRect::horizontal(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pt-") {
                smallvec![(
                    &element_attrs::padding as _,
                    smallbox!(UiOptionalRect::top(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pb-") {
                smallvec![(
                    &element_attrs::padding as _,
                    smallbox!(UiOptionalRect::bottom(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pl-") {
                smallvec![(
                    &element_attrs::padding as _,
                    smallbox!(UiOptionalRect::left(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pr-") {
                smallvec![(
                    &element_attrs::padding as _,
                    smallbox!(UiOptionalRect::right(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("m-") {
                smallvec![(
                    &element_attrs::margin as _,
                    smallbox!(UiOptionalRect::all(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("my-") {
                smallvec![(
                    &element_attrs::margin as _,
                    smallbox!(UiOptionalRect::vertical(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mx-") {
                smallvec![(
                    &element_attrs::margin as _,
                    smallbox!(UiOptionalRect::horizontal(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mt-") {
                smallvec![(
                    &element_attrs::margin as _,
                    smallbox!(UiOptionalRect::top(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mb-") {
                smallvec![(
                    &element_attrs::margin as _,
                    smallbox!(UiOptionalRect::bottom(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("ml-") {
                smallvec![(
                    &element_attrs::margin as _,
                    smallbox!(UiOptionalRect::left(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mr-") {
                smallvec![(
                    &element_attrs::margin as _,
                    smallbox!(UiOptionalRect::right(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("border") {
                if class == "" {
                    smallvec![(
                        &element_attrs::border as _,
                        smallbox!(UiOptionalRect::all(Val::Px(1.0)))
                    ),]
                } else if let Some(class) = class.strip_prefix("-") {
                    if let Some(color) = parse_color(class) {
                        smallvec![(
                            &element_attrs::border_color as _,
                            smallbox!(BorderColor(color))
                        ),]
                    } else if let Ok(size) = class.parse::<f32>() {
                        smallvec![(
                            &element_attrs::border as _,
                            smallbox!(UiOptionalRect::all(Val::Px(size as f32)))
                        ),]
                    } else if let Some(class) = class.strip_prefix("t") {
                        smallvec![(
                            &element_attrs::border as _,
                            smallbox!(parse_border_size(class, UiOptionalRect::top))
                        ),]
                    } else if let Some(class) = class.strip_prefix("r") {
                        smallvec![(
                            &element_attrs::border as _,
                            smallbox!(parse_border_size(class, UiOptionalRect::right))
                        ),]
                    } else if let Some(class) = class.strip_prefix("b") {
                        smallvec![(
                            &element_attrs::border as _,
                            smallbox!(parse_border_size(class, UiOptionalRect::bottom))
                        ),]
                    } else if let Some(class) = class.strip_prefix("l") {
                        smallvec![(
                            &element_attrs::border as _,
                            smallbox!(parse_border_size(class, UiOptionalRect::left))
                        ),]
                    } else {
                        default()
                    }
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("overflow-") {
                smallvec![(
                    &element_attrs::border as _,
                    smallbox!(match class {
                        "clip" => OptionalOverflow {
                            x: Some(OverflowAxis::Clip),
                            y: Some(OverflowAxis::Clip),
                        },
                        _ => OptionalOverflow {
                            x: Some(OverflowAxis::Visible),
                            y: Some(OverflowAxis::Visible),
                        },
                    })
                ),]
            } else if let Some(class) = class.strip_prefix("overflow-x-") {
                smallvec![(
                    &element_attrs::border as _,
                    smallbox!(OptionalOverflow {
                        x: Some(match class {
                            "clip" => OverflowAxis::Clip,
                            _ => OverflowAxis::Visible,
                        }),
                        ..default()
                    })
                ),]
            } else if let Some(class) = class.strip_prefix("overflow-y-") {
                smallvec![(
                    &element_attrs::border as _,
                    smallbox!(OptionalOverflow {
                        y: Some(match class {
                            "clip" => OverflowAxis::Clip,
                            _ => OverflowAxis::Visible,
                        }),
                        ..default()
                    })
                ),]
            } else {
                default()
            }
        }
    }
}

pub fn parse_class<'a>(class: &'a str) -> TailwindClassItem {
    if let Some(class) = class.strip_prefix("hover:") {
        TailwindClassItem(parse_class_inner(class), Interaction::Hovered)
    } else if let Some(class) = class.strip_prefix("active:") {
        TailwindClassItem(parse_class_inner(class), Interaction::Pressed)
    } else {
        TailwindClassItem(parse_class_inner(class), default())
    }
}

fn parse_border_size(class: &str, f: fn(val: Val) -> UiOptionalRect) -> UiOptionalRect {
    if class == "" {
        f(Val::Px(1.0))
    } else if let Some(class) = class.strip_prefix("-") {
        f(parse_size_val(class))
    } else {
        UiOptionalRect::default_value()
    }
}
