use bevy::prelude::{Component, Visibility};
use bevy::reflect::Reflect;
use bevy::ui::*;
use bevy::utils::{default, HashMap};
use smallvec::{smallvec, SmallVec};
use std::ops::Deref;

use crate::smallbox::S1;
use crate::{schema_props, SchemaPropUntyped, SchemaTypeUnTyped, SetAttrValueContext};
use crate::{smallbox, SmallBox};
use crate::{try_get_schema_type, OptionalOverflow, PropValue, Texture, UiOptionalRect};

mod colors;

use crate::prelude::warn;
pub use colors::*;

#[derive(Default)]
pub struct TailwindClassItem(
    pub SmallVec<[(&'static dyn SchemaPropUntyped, SmallBox<dyn PropValue, S1>); 4]>,
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
        let left = other.1;
        let right = n.1;
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
    let mut interaction_prop_set: u64 = 0;
    for (_index, item) in items {
        if item.1 != Interaction::None {
            interaction_classes.push(Clone::clone(&item));
        }

        for (prop, value) in item.0.into_iter() {
            let prop_is_no_set = set_bits & !(1 << prop.index()) == set_bits;
            if item.1 != Interaction::None {
                interaction_prop_set |= (1 << prop.index());
            }
            match (item.1, interaction) {
                (Interaction::Pressed, Interaction::Pressed) => {
                    if prop_is_no_set {
                        prop.set_dyn_value_in_class(context, value);
                        set_bits |= (1 << prop.index());
                    }
                }
                // (Interaction::Pressed,_) => {
                // }
                (Interaction::Hovered, Interaction::Hovered | Interaction::Pressed) => {
                    if prop_is_no_set {
                        prop.set_dyn_value_in_class(context, value);
                        set_bits |= (1 << prop.index());
                    }
                }
                (Interaction::None, _) => {
                    if interaction_prop_set & !(1 << prop.index()) != interaction_prop_set {
                        normal_props_map.insert(prop.index(), value.clone());
                    }
                    if prop_is_no_set {
                        prop.set_dyn_value_in_class(context, value);
                        set_bits |= (1 << prop.index());
                    }
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
    let schema_type: &dyn SchemaTypeUnTyped =
        try_get_schema_type(entity_extra_data.schema_name).unwrap();

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
    let schema_type: &dyn SchemaTypeUnTyped =
        try_get_schema_type(context.entity_extra_data().schema_name).unwrap();

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
) -> SmallVec<[(&'static dyn SchemaPropUntyped, SmallBox<dyn PropValue, S1>); 4]> {
    match class {
        "visible" => smallvec![(
            &schema_props::visibility as _,
            smallbox!(Visibility::Visible)
        ),],
        "invisible" => smallvec![(
            &schema_props::visibility as _,
            smallbox!(Visibility::Hidden)
        ),],
        "flex-col" => smallvec![
            (&schema_props::display as _, smallbox!(Display::Flex)),
            (
                &schema_props::flex_direction as _,
                smallbox!(FlexDirection::Column)
            )
        ],
        "flex-row" => smallvec![
            (&schema_props::display as _, smallbox!(Display::Flex)),
            (
                &schema_props::flex_direction as _,
                smallbox!(FlexDirection::Row)
            )
        ],
        "grid" => smallvec![(&schema_props::display as _, smallbox!(Display::Grid)),],
        "shrink" => smallvec![(&schema_props::flex_shrink as _, smallbox!(1.0)),],
        "shrink-0" => smallvec![(&schema_props::flex_shrink as _, smallbox!(0.0)),],
        "grow" => smallvec![(&schema_props::flex_grow as _, smallbox!(1.0)),],
        "grow-0" => smallvec![(&schema_props::flex_grow as _, smallbox!(0.0)),],
        class => {
            if let Some(index) = class.strip_prefix("z-") {
                if let Ok(index) = index.parse() {
                    smallvec![(
                        &schema_props::z_index as _,
                        smallbox!(ZIndex::Global(index))
                    ),]
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("justify-") {
                smallvec![(
                    &schema_props::justify_content as _,
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
                    &schema_props::align_items as _,
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
                    (&schema_props::column_gap as _, smallbox!(gap)),
                    (&schema_props::row_gap as _, smallbox!(gap)),
                ]
            } else if let Some(class) = class.strip_prefix("gap-x-") {
                let gap = parse_size_val(class);
                smallvec![(&schema_props::row_gap as _, smallbox!(gap)),]
            } else if let Some(class) = class.strip_prefix("gap-y-") {
                let gap = parse_size_val(class);
                smallvec![(&schema_props::column_gap as _, smallbox!(gap)),]
            } else if class == "relative" {
                smallvec![(
                    &schema_props::position_type as _,
                    smallbox!(PositionType::Relative)
                ),]
            } else if class == "absolute" {
                smallvec![(
                    &schema_props::position_type as _,
                    smallbox!(PositionType::Absolute)
                ),]
            } else if class == "hidden" {
                smallvec![(&schema_props::display as _, smallbox!(Display::None)),]
            } else if let Some(class) = class.strip_prefix("left-") {
                smallvec![(&schema_props::left as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("right-") {
                smallvec![(&schema_props::right as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("top-") {
                smallvec![(&schema_props::top as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("bottom-") {
                smallvec![(&schema_props::bottom as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("flex-") {
                if let Some(val) = (match class {
                    "wrap" => Some(FlexWrap::Wrap),
                    "wrap-reverse" => Some(FlexWrap::WrapReverse),
                    "nowrap" => Some(FlexWrap::NoWrap),
                    _ => None,
                }) {
                    smallvec![(&schema_props::flex_wrap as _, smallbox!(val))]
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("w-") {
                smallvec![(&schema_props::width as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("h-") {
                smallvec![(&schema_props::height as _, smallbox!(parse_size_val(class))),]
            } else if let Some(class) = class.strip_prefix("min-w-") {
                smallvec![(
                    &schema_props::min_width as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("min-h-") {
                smallvec![(
                    &schema_props::min_height as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("max-w-") {
                smallvec![(
                    &schema_props::min_width as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("max-h-") {
                smallvec![(
                    &schema_props::max_height as _,
                    smallbox!(parse_size_val(class))
                ),]
            } else if let Some(class) = class.strip_prefix("bg-") {
                if let Some(color) = parse_color(class) {
                    smallvec![(
                        &schema_props::background as _,
                        smallbox!(Texture::Color(color))
                    ),]
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("text-") {
                if let Some(color) = parse_color(class) {
                    smallvec![(&schema_props::text_color as _, smallbox!(color)),]
                } else if let Ok(size) = class.parse::<f32>() {
                    smallvec![(&schema_props::font_size as _, smallbox!(size)),]
                } else {
                    default()
                }
            } else if let Some(class) = class.strip_prefix("p-") {
                let padding = parse_size_val(class);
                smallvec![(
                    &schema_props::padding as _,
                    smallbox!(UiOptionalRect::all(padding))
                ),]
            } else if let Some(class) = class.strip_prefix("py-") {
                smallvec![(
                    &schema_props::padding as _,
                    smallbox!(UiOptionalRect::vertical(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("px-") {
                smallvec![(
                    &schema_props::padding as _,
                    smallbox!(UiOptionalRect::horizontal(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pt-") {
                smallvec![(
                    &schema_props::padding as _,
                    smallbox!(UiOptionalRect::top(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pb-") {
                smallvec![(
                    &schema_props::padding as _,
                    smallbox!(UiOptionalRect::bottom(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pl-") {
                smallvec![(
                    &schema_props::padding as _,
                    smallbox!(UiOptionalRect::left(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("pr-") {
                smallvec![(
                    &schema_props::padding as _,
                    smallbox!(UiOptionalRect::right(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("m-") {
                smallvec![(
                    &schema_props::margin as _,
                    smallbox!(UiOptionalRect::all(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("my-") {
                smallvec![(
                    &schema_props::margin as _,
                    smallbox!(UiOptionalRect::vertical(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mx-") {
                smallvec![(
                    &schema_props::margin as _,
                    smallbox!(UiOptionalRect::horizontal(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mt-") {
                smallvec![(
                    &schema_props::margin as _,
                    smallbox!(UiOptionalRect::top(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mb-") {
                smallvec![(
                    &schema_props::margin as _,
                    smallbox!(UiOptionalRect::bottom(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("ml-") {
                smallvec![(
                    &schema_props::margin as _,
                    smallbox!(UiOptionalRect::left(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("mr-") {
                smallvec![(
                    &schema_props::margin as _,
                    smallbox!(UiOptionalRect::right(parse_size_val(class)))
                ),]
            } else if let Some(class) = class.strip_prefix("border") {
                if class == "" {
                    smallvec![(
                        &schema_props::border as _,
                        smallbox!(UiOptionalRect::all(Val::Px(1.0)))
                    ),]
                } else if let Some(class) = class.strip_prefix("-") {
                    if let Some(color) = parse_color(class) {
                        smallvec![(
                            &schema_props::border_color as _,
                            smallbox!(BorderColor(color))
                        ),]
                    } else if let Some(class) = class.strip_prefix("t") {
                        smallvec![(
                            &schema_props::border as _,
                            smallbox!(parse_border_size(class, UiOptionalRect::top))
                        ),]
                    } else if let Some(class) = class.strip_prefix("r") {
                        smallvec![(
                            &schema_props::border as _,
                            smallbox!(parse_border_size(class, UiOptionalRect::right))
                        ),]
                    } else if let Some(class) = class.strip_prefix("b") {
                        smallvec![(
                            &schema_props::border as _,
                            smallbox!(parse_border_size(class, UiOptionalRect::bottom))
                        ),]
                    } else if let Some(class) = class.strip_prefix("l") {
                        smallvec![(
                            &schema_props::border as _,
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
                    &schema_props::border as _,
                    smallbox!(match class {
                        "hidden" => OptionalOverflow {
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
                    &schema_props::border as _,
                    smallbox!(OptionalOverflow {
                        x: Some(match class {
                            "hidden" => OverflowAxis::Clip,
                            _ => OverflowAxis::Visible,
                        }),
                        ..default()
                    })
                ),]
            } else if let Some(class) = class.strip_prefix("overflow-y-") {
                smallvec![(
                    &schema_props::border as _,
                    smallbox!(OptionalOverflow {
                        y: Some(match class {
                            "hidden" => OverflowAxis::Clip,
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
        UiOptionalRect::default()
    }
}
