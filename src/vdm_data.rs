use bevy::hierarchy::{Children, Parent};
use bevy::prelude::{default, Entity, Resource, World};
use bevy::utils::HashMap;
use dioxus::core::ElementId;
use smallvec::SmallVec;

pub const MAX_CHILDREN: usize = 1024;

#[derive(Resource, Default)]
pub struct TemplateData {
    pub template_name_to_entities: HashMap<String, Entity>,
}

#[derive(Resource, Default)]
pub struct VDomData {
    pub loaded_node_stack: SmallVec<[Entity; MAX_CHILDREN]>,
    pub element_id_to_entity: HashMap<ElementId, Entity>,
}

impl VDomData {
    pub fn new() -> Self {
        Self {
            loaded_node_stack: default(),
            element_id_to_entity: default(),
        }
    }

    pub fn get_element_id_by_entity(&self, entity: Entity) -> Option<ElementId> {
        self.element_id_to_entity
            .iter()
            .find(|(_, n)| **n == entity)
            .map(|n| *n.0)
    }

    pub fn split_stack(&mut self, at: usize) -> SmallVec<[Entity; MAX_CHILDREN]> {
        if at > self.loaded_node_stack.len() {
            let len = self.loaded_node_stack.len();
            panic!("`at` split index (is {at}) should be <= len (is {len})");
        }

        if at == 0 {
            let cap = self.loaded_node_stack.capacity();
            return std::mem::replace(
                &mut self.loaded_node_stack,
                SmallVec::<[Entity; MAX_CHILDREN]>::with_capacity(cap),
            );
        }

        let other_len = self.loaded_node_stack.len() - at;
        let mut other = SmallVec::<[Entity; MAX_CHILDREN]>::with_capacity(other_len);

        unsafe {
            self.loaded_node_stack.set_len(at);
            other.set_len(other_len);

            std::ptr::copy_nonoverlapping(
                self.loaded_node_stack.as_ptr().add(at),
                other.as_mut_ptr(),
                other_len,
            );
        }

        other
    }

    pub fn load_path(
        &self,
        path: &[u8],
        world: &mut World,
        read_parent: bool,
    ) -> (Entity, Option<Entity>) {
        let mut current_entity = *self.loaded_node_stack.last().unwrap();
        let mut parent_id = None;

        for index in path {
            let children = world.get::<Children>(current_entity).unwrap();
            let child_entity = children[*index as usize];
            parent_id = Some(current_entity);
            current_entity = child_entity
        }
        if read_parent && parent_id.is_none() {
            let parent = world.get::<Parent>(current_entity).unwrap();
            parent_id = Some(parent.get());
        }
        (current_entity, parent_id)
    }
}
