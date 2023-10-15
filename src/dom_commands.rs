use std::ops::DerefMut;

use bevy::core::Name;
use bevy::ecs::system::Command;
use bevy::hierarchy::BuildWorldChildren;
use bevy::prelude::{
    default, error, AppTypeRegistry, Color, DespawnRecursiveExt, Entity, Mut, NodeBundle, Reflect,
    SpatialBundle, Text, TextBundle, TextSection, TextStyle, Visibility, World,
};
use dioxus::core::ElementId;

use crate::dom_template::{DomTemplate, DomTemplateAttribute, DomTemplateNode};
use crate::ecs_fns::{insert_after, insert_before, WorldExtension};
use crate::entity_extra_data::{EntitiesExtraData, EntityExtraData};
use crate::prelude::dioxus_elements::events::{
    listen_dom_event_by_name, unlisten_dom_event_by_name,
};
use crate::prelude::warn;
use crate::vdm_data::{TemplateData, VDomData};
use crate::{
    ecs_fns, elements, get_element_type, ElementTypeBase, NodeTemplate, SetAttrValueContext,
    TemplateWorld,
};

pub fn create_template_node(
    template_world: &mut World,
    entities_extra_data: &mut EntitiesExtraData,
    template_node: DomTemplateNode,
    type_registry: AppTypeRegistry,
) -> Entity {
    match template_node {
        DomTemplateNode::Element {
            children,
            attrs,
            tag,
            ..
        } => {
            let entities = {
                let mut entities = vec![];
                entities.reserve(children.len());
                for n in children.into_iter() {
                    entities.push(create_template_node(
                        template_world,
                        entities_extra_data,
                        n,
                        type_registry.clone(),
                    ));
                }
                entities
            };

            let static_attrs = attrs
                .into_iter()
                .filter_map(|attr| {
                    if let DomTemplateAttribute::Static { name, value, .. } = attr {
                        Some((name, value))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let schema_type = get_element_type(tag);
            let mut entity_ref = schema_type.spawn(template_world);
            let entity = entity_ref.id();
            let mut entity_extra_data = EntityExtraData::new(tag);
            for (name, _) in static_attrs.iter() {
                let Some(attr) = schema_type.attr(&name) else {
                    warn!("no found attr {:?}", name);
                    continue;
                };
                entity_extra_data.set_attr(attr.index(), true);
            }
            entity_ref.push_children(entities.as_slice());
            entities_extra_data.insert(entity, entity_extra_data);
            let mut context = SetAttrValueContext {
                entity_ref: &mut entity_ref,
                entities_extra_data,
                type_registry,
            };
            for (name, value) in static_attrs.into_iter() {
                let Some(attr) = schema_type.attr(&name) else {
                    warn!("no found attr {:?}", name);
                    continue;
                };
                attr.set_by_attr_value(&mut context, DomAttributeValue::Text(value));
            }

            entity
        }
        DomTemplateNode::Text {
            text: text_value, ..
        } => {
            let entity_ref = template_world.spawn(
                (TextBundle {
                    text: Text::from_section(
                        text_value,
                        TextStyle {
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    ..default()
                }),
            );
            entities_extra_data.insert(entity_ref.id(), EntityExtraData::new(elements::text::NAME));
            entity_ref.id()
        }
        DomTemplateNode::Dynamic { id } => {
            // todo: schema tag?
            let entity = template_world.spawn((
                NodeBundle::default(),
                Name::new(format!("Dynamic, ElementId {:?}", id)),
            ));
            entities_extra_data.empty_node_entities.push(entity.id());
            entity.id()
        }
        DomTemplateNode::DynamicText { .. } => {
            let entity_ref = template_world.spawn(TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                ..default()
            });
            entities_extra_data.insert(entity_ref.id(), EntityExtraData::new(elements::text::NAME));
            entity_ref.id()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CreateTemplates {
    pub templates: Vec<DomTemplate>,
}

impl Command for CreateTemplates {
    fn apply(self, world: &mut World) {
        let type_registry = world.resource::<AppTypeRegistry>().clone();
        let mut template_world = world.resource_mut::<TemplateWorld>();
        template_world.resource_scope(
            |template_world, mut entities_extra_data: Mut<EntitiesExtraData>| {
                template_world.resource_scope(
                    |template_world, mut template_data: Mut<TemplateData>| {
                        for template in self.templates.into_iter() {
                            let mut entities = vec![];
                            for n in template.roots.into_iter() {
                                entities.push(create_template_node(
                                    template_world,
                                    entities_extra_data.as_mut(),
                                    n,
                                    type_registry.clone(),
                                ));
                            }

                            let mut template_entity_ref = template_world.spawn((
                                SpatialBundle {
                                    visibility: Visibility::Hidden,
                                    ..default()
                                },
                                Name::new(format!("template {}", template.name.clone())),
                                NodeTemplate,
                            ));
                            let template_entity = template_entity_ref.id();
                            template_entity_ref.push_children(entities.as_slice());
                            template_data
                                .template_name_to_entities
                                .insert(template.name, template_entity);
                        }
                    },
                )
            },
        );
    }
}

#[derive(Clone, Debug, Default)]
pub struct LoadTemplate {
    pub element_id: ElementId,
    pub name: String,
    pub root_index: usize,
}

impl Command for LoadTemplate {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut template_world: Mut<TemplateWorld>| {
            template_world.resource_scope(|template_world, template_data: Mut<TemplateData>| {
                let template_entity = template_data.template_name_to_entities[&self.name];
                let root_entity =
                    template_world.get_child_by_index(template_entity, self.root_index);
                let loaded_entity = template_world.resource_scope(
                    |template_world, mut template_entities_extra_data: Mut<EntitiesExtraData>| {
                        world.resource_scope(
                            |world, mut entities_extra_data: Mut<EntitiesExtraData>| {
                                ecs_fns::clone_entity_nest(
                                    world,
                                    entities_extra_data.as_mut(),
                                    template_world,
                                    template_entities_extra_data.as_mut(),
                                    root_entity,
                                )
                            },
                        )
                    },
                );
                world.resource_scope(|_world, mut vdom_data: Mut<VDomData>| {
                    vdom_data.loaded_node_stack.push(loaded_entity);
                    vdom_data
                        .element_id_to_entity
                        .insert(self.element_id, loaded_entity);
                });
            });
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct AssignId {
    pub id: ElementId,
    pub path: &'static [u8],
}

impl Command for AssignId {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let (entity_id, _) = vdom_data.load_path(self.path, world, false);
            vdom_data.element_id_to_entity.insert(self.id, entity_id);
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct AppendChildren {
    pub stack_pop_count: usize,
    pub id: ElementId,
}

impl Command for AppendChildren {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let at = vdom_data.loaded_node_stack.len() - self.stack_pop_count;

            let children = vdom_data.split_stack(at);

            let parent = vdom_data.element_id_to_entity[&self.id];
            let mut parent_ref = world.entity_mut(parent);
            parent_ref.push_children(&children);
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct PushRoot {
    pub id: ElementId,
}

impl Command for PushRoot {
    fn apply(self, world: &mut World) {
        world.resource_scope(|_world, mut vdom_data: Mut<VDomData>| {
            let entity = vdom_data.element_id_to_entity[&self.id];
            vdom_data.loaded_node_stack.push(entity);
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct Remove {
    pub id: ElementId,
}

impl Command for Remove {
    fn apply(self, world: &mut World) {
        let vdom_data = world.resource::<VDomData>();
        let entity = vdom_data.element_id_to_entity[&self.id];
        world.entity_mut(entity).despawn_recursive();
    }
}

#[derive(Clone, Debug, Default)]
pub struct HydrateText {
    pub path: &'static [u8],
    pub value: String,
    pub id: ElementId,
}

impl Command for HydrateText {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let (entity, _) = vdom_data.load_path(self.path, world, false);
            vdom_data.element_id_to_entity.insert(self.id, entity);
            let mut entity_mut = world.entity_mut(entity);
            if let Some(mut text) = entity_mut.get_mut::<Text>() {
                text.sections = vec![TextSection::new(
                    self.value,
                    text.sections
                        .first()
                        .map(|n| n.style.clone())
                        .unwrap_or_default(),
                )];
            } else {
                entity_mut.insert(Text::from_section(
                    self.value,
                    TextStyle {
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            }
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct SetText {
    pub value: String,
    pub id: ElementId,
}

impl Command for SetText {
    fn apply(self, world: &mut World) {
        let vdom_data = world.resource::<VDomData>();
        let entity = vdom_data.element_id_to_entity[&self.id];
        let mut text = world.get_mut::<Text>(entity).unwrap();

        text.sections = vec![TextSection::new(
            self.value,
            text.sections
                .first()
                .map(|n| n.style.clone())
                .unwrap_or_default(),
        )];
    }
}

#[derive(Clone, Debug, Default)]
pub struct InsertAfter {
    pub stack_pop_count: usize,
    pub id: ElementId,
}

impl Command for InsertAfter {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let at = vdom_data.loaded_node_stack.len() - self.stack_pop_count;
            let new_entities = vdom_data.split_stack(at);

            let entity = vdom_data.element_id_to_entity[&self.id];
            insert_after(world, entity, &new_entities);
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct InsertBefore {
    pub stack_pop_count: usize,
    pub id: ElementId,
}

impl Command for InsertBefore {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let at = vdom_data.loaded_node_stack.len() - self.stack_pop_count;
            let new_entities = vdom_data.split_stack(at);

            let entity = vdom_data.element_id_to_entity[&self.id];
            insert_before(world, entity, &new_entities);
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct ReplaceWith {
    pub stack_pop_count: usize,
    pub id: ElementId,
}

impl Command for ReplaceWith {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let at = vdom_data.loaded_node_stack.len() - self.stack_pop_count;
            let new_entities = vdom_data.split_stack(at);

            let old_entity = vdom_data.element_id_to_entity[&self.id];

            insert_before(world, old_entity, &new_entities);

            world.entity_mut(old_entity).despawn_recursive();
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct ReplacePlaceholder {
    pub path: &'static [u8],
    pub stack_pop_count: usize,
}

impl Command for ReplacePlaceholder {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let at = vdom_data.loaded_node_stack.len() - self.stack_pop_count;
            let new_entities = vdom_data.split_stack(at);
            let (old_entity, parent) = vdom_data.load_path(self.path, world, true);
            let old_entity_index = *self.path.last().unwrap() as usize;

            let mut parent_ref = world.entity_mut(parent.unwrap());
            parent_ref.insert_children(old_entity_index, &new_entities);

            world.entity_mut(old_entity).despawn_recursive();
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct CreatePlaceholder {
    pub id: ElementId,
}

impl Command for CreatePlaceholder {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let placeholder_entity_ref =
                world.spawn((Name::new("[placeholder]"), NodeBundle::default()));
            let placeholder_entity = placeholder_entity_ref.id();
            vdom_data
                .element_id_to_entity
                .insert(self.id, placeholder_entity);
            vdom_data.loaded_node_stack.push(placeholder_entity);
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct CreateTextNode {
    pub id: ElementId,
    pub value: String,
}

impl Command for CreateTextNode {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut vdom_data: Mut<VDomData>| {
            let text_entity_ref = world.spawn((TextBundle {
                text: Text::from_section(
                    self.value,
                    TextStyle {
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                ..default()
            },));
            let text_entity = text_entity_ref.id();
            vdom_data.element_id_to_entity.insert(self.id, text_entity);
            vdom_data.loaded_node_stack.push(text_entity);
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct NewEventListener {
    pub id: ElementId,
    pub name: String,
}

impl Command for NewEventListener {
    fn apply(self, world: &mut World) {
        listen_dom_event_by_name(world, self.id, &self.name);
    }
}

#[derive(Clone, Debug, Default)]
pub struct RemoveEventListener {
    pub id: ElementId,
    pub name: String,
}

impl Command for RemoveEventListener {
    fn apply(self, world: &mut World) {
        unlisten_dom_event_by_name(world, self.id, &self.name);
    }
}

#[derive(Default)]
pub enum DomAttributeValue {
    Text(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Any(Box<dyn Reflect>),
    #[default]
    None,
}

impl From<DomAttributeValue> for Option<f64> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Float(value) => Some(value),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<bool> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Bool(value) => Some(value),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<i64> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Int(value) => Some(value),
            _ => None,
        }
    }
}

impl From<DomAttributeValue> for Option<f32> {
    fn from(value: DomAttributeValue) -> Self {
        match value {
            DomAttributeValue::Text(value) => Some(value.parse().unwrap_or_default()),
            DomAttributeValue::Int(value) => Some(value as f32),
            DomAttributeValue::Float(value) => Some(value as f32),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct SetAttribute {
    pub name: &'static str,
    pub value: DomAttributeValue,
    pub id: ElementId,
}

impl Command for SetAttribute {
    fn apply(self, world: &mut World) {
        let vdom_data = world.resource::<VDomData>();
        let type_registry = world.resource::<AppTypeRegistry>().clone();
        let entity = vdom_data.element_id_to_entity[&self.id];

        // todo: handle set attr value error

        world.resource_scope(|world, mut entities_extra_data: Mut<EntitiesExtraData>| {
            let Some(entity_extra_data) = entities_extra_data.get_mut(&entity) else {
                error!("No Found EntityExtraData by {:#?}", entity);
                return;
            };
            let schema_type = get_element_type(entity_extra_data.schema_name);
            match schema_type.attr(self.name) {
                Some(attr) => {
                    entity_extra_data
                        .set_attr(attr.index(), !matches!(self.value, DomAttributeValue::None));

                    let mut entity_ref = world.entity_mut(entity);

                    attr.set_by_attr_value(
                        &mut SetAttrValueContext {
                            entity_ref: &mut entity_ref,
                            entities_extra_data: entities_extra_data.deref_mut(),
                            type_registry,
                        },
                        self.value,
                    );
                }
                None => {
                    let Some(attr) = schema_type.composite_attr(self.name) else {
                        error!("No Found Attr by {:#?}", self.name);
                        return;
                    };
                    let value_is_some = !matches!(self.value, DomAttributeValue::None);
                    let mut entity_ref = world.entity_mut(entity);

                    let Some(attrs) = attr.set_by_attr_value_and_get_attrs(
                        &mut SetAttrValueContext {
                            entity_ref: &mut entity_ref,
                            entities_extra_data: entities_extra_data.deref_mut(),
                            type_registry,
                        },
                        self.value,
                    ) else {
                        return;
                    };

                    let Some(entity_extra_data) = entities_extra_data.get_mut(&entity) else {
                        error!("No Found EntityExtraData by {:#?}", entity);
                        return;
                    };
                    for attr in attrs {
                        entity_extra_data.set_attr(attr.index(), value_is_some);
                    }
                }
            }
        });
    }
}
