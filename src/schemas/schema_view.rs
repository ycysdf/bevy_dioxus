#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::prelude::World;
use crate::{impl_schema_type_base, SchemaType};
use bevy::ecs::world::EntityMut;
use bevy::prelude::NodeBundle;

impl_schema_type_base!(view);

impl SchemaType for view {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        world.spawn(NodeBundle::default())
    }
}
