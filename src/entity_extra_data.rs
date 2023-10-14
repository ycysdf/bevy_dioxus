use std::ops::{Deref, DerefMut};

use bevy::prelude::{Entity, Resource};
use bevy::utils::{default, HashMap};
use smallvec::SmallVec;

use crate::element_core::AttrValue;
use crate::smallbox::S1;
use crate::tailwind::TailwindClassItem;
use crate::SmallBox;

pub type AttrSetBits = u64;
pub type AttrIndex = u8;

pub fn get_all_prop_indecs() -> impl Iterator<Item = AttrIndex> {
    let bit_count = (std::mem::size_of::<AttrSetBits>() * 8) as AttrIndex;
    0..bit_count
}

#[derive(Clone)]
pub struct EntityExtraData {
    pub schema_name: &'static str,
    pub attr_is_set: AttrSetBits,
    pub class_attr_is_set: AttrSetBits,
    class_attr_set_count: u8,
    pub interaction_classes: SmallVec<[TailwindClassItem; 8]>,
    pub normal_props_map: HashMap<u8, SmallBox<dyn AttrValue, S1>>,
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
    pub fn set_attr(&mut self, attr_index: AttrIndex, is_set: bool) {
        if attr_index == 0 {
            return;
        }
        if is_set {
            self.attr_is_set |= (1 << attr_index);
        } else {
            self.attr_is_set &= !(1 << attr_index);
        }
    }

    pub fn is_set_attr(&self, attr_index: AttrIndex) -> bool {
        self.attr_is_set & (!(1 << attr_index)) != self.attr_is_set
    }

    pub fn set_class_attr(&mut self, attr_index: AttrIndex, is_set: bool) {
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

    pub fn is_set_class_attr(&self, attr_index: AttrIndex) -> bool {
        self.class_attr_is_set & (!(1 << attr_index)) != self.class_attr_is_set
    }

    pub fn iter_set_class_attr_indices(&self) -> impl Iterator<Item = AttrIndex> + 'static {
        let num = self.class_attr_is_set;
        get_all_prop_indecs()
            .filter(move |i| (num >> i) & 1 == 1)
            .take(self.class_attr_set_count as usize)
    }
    pub fn iter_class_attr_indices_exclude(&self, bits: AttrSetBits) -> impl Iterator<Item = AttrIndex> + 'static {
        let num = self.class_attr_is_set & !bits;
        get_all_prop_indecs().filter(move |i| (num >> i) & 1 == 1)
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
