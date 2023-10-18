#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::NodeBundle;
use crate::view;
use crate::ElementType;
use crate::prelude::World;

impl ElementType for view {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        world.spawn(NodeBundle::default())
    }
}
