use std::ops::{Deref, DerefMut};

use bevy::prelude::{Entity, Resource};
use bevy::utils::{default, HashMap};
use smallvec::SmallVec;

use crate::{PropValue, SmallBox};
use crate::smallbox::S1;
use crate::tailwind::TailwindClassItem;

#[derive(Clone)]
pub struct EntityExtraData {
    pub schema_name: &'static str,
    pub attr_is_set: u64,
    pub class_attr_is_set: u64,
    class_attr_set_count: u8,
    pub interaction_classes: SmallVec<[TailwindClassItem; 8]>,
    pub normal_props_map: HashMap<u8, SmallBox<dyn PropValue, S1>>,
}

impl EntityExtraData {
    pub fn new(schema_name: &'static str) -> Self {
        Self {
            schema_name,
            attr_is_set: 0,
            class_attr_is_set: 0,
            class_attr_set_count: 0,
            interaction_classes: default(),
            normal_props_map: default(),
        }
    }
    pub fn set_attr(&mut self, attr_index: u8, is_set: bool) {
        if attr_index == 0 {
            return;
        }
        if is_set {
            self.attr_is_set |= (1 << attr_index);
        } else {
            self.attr_is_set &= !(1 << attr_index);
        }
    }

    pub fn is_set_attr(&self, attr_index: u8) -> bool {
        self.attr_is_set & (!(1 << attr_index)) != self.attr_is_set
    }

    pub fn set_class_attr(&mut self, attr_index: u8, is_set: bool) {
        if is_set == self.is_set_class_attr(attr_index) {
            return;
        }
        if is_set {
            self.class_attr_is_set |= 1 << attr_index;
            self.class_attr_set_count += 1;
        } else {
            self.class_attr_is_set &= !(1 << attr_index);
            self.class_attr_set_count -= 1;
        }
    }

    pub fn is_set_class_attr(&self, attr_index: u8) -> bool {
        self.class_attr_is_set & (!(1 << attr_index)) != self.class_attr_is_set
    }

    pub fn iter_set_class_attr_indices(&self) -> impl Iterator<Item=u8> + 'static {
        let num = self.class_attr_is_set;
        (0..64)
            .filter(move |i| (num >> i) & 1 == 1)
            .take(self.class_attr_set_count as usize)
    }
    pub fn iter_class_attr_indices_exclude(&self, bits: u64) -> impl Iterator<Item=u8> + 'static {
        let num = self.class_attr_is_set & !bits;
        (0..64).filter(move |i| (num >> i) & 1 == 1)
    }
}

#[derive(Resource, Default)]
pub struct EntitiesExtraData {
    inner: HashMap<Entity, EntityExtraData>,
    pub empty_node_entities: Vec<Entity>,
}

impl Deref for EntitiesExtraData {
    type Target = HashMap<Entity, EntityExtraData>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for EntitiesExtraData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
