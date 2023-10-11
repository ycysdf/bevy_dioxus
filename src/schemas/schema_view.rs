#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::NodeBundle;

use crate::{impl_schema_type_base, SchemaType};
use crate::prelude::World;

impl_schema_type_base!(view);

impl SchemaType for view {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        world.spawn(NodeBundle::default())
    }
}
