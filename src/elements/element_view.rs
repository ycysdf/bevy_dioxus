#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::NodeBundle;
use bevy::reflect::Reflect;

use crate::{ElementType, impl_element_type_base};
use crate::prelude::World;

impl_element_type_base!(
    #[derive(Reflect, Debug, Clone, Copy)]
    view
);

impl ElementType for view {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        world.spawn(NodeBundle::default())
    }
}
