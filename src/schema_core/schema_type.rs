use bevy::ecs::world::EntityMut;
use bevy::prelude::World;
use crate::schema_core::SchemaPropUntyped;

pub trait SchemaTypeUnTyped {
    fn schema_name(&self) -> &'static str;
    fn namespace(&self) -> Option<&'static str>;

    fn props(&self) -> &'static [&'static dyn SchemaPropUntyped];

    fn prop(&self, attr_name: &str) -> Option<&'static dyn SchemaPropUntyped>;
    fn prop_by_index(&self, index: u8) -> &'static dyn SchemaPropUntyped {
        self.props()[index as usize]
    }
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w>;
}

impl<T: SchemaTypeBase + SchemaType> SchemaTypeUnTyped for T {
    fn schema_name(&self) -> &'static str {
        T::TAG_NAME
    }

    fn namespace(&self) -> Option<&'static str> {
        T::NAME_SPACE
    }

    fn props(&self) -> &'static [&'static dyn SchemaPropUntyped] {
        T::PROPS
    }

    fn prop(&self, attr_name: &str) -> Option<&'static dyn SchemaPropUntyped> {
        T::prop(attr_name)
    }
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w> {
        self.spawn(world)
    }
}

pub trait SchemaTypeBase {
    const TAG_NAME: &'static str;
    const NAME_SPACE: Option<&'static str> = None;
    const PROPS: &'static [&'static dyn SchemaPropUntyped];
    const NAME: &'static str = Self::TAG_NAME;

    fn prop(attr_name: &str) -> Option<&'static dyn SchemaPropUntyped> {
        Self::PROPS.iter().find(|n| n.name() == attr_name).copied()
    }
}


pub trait SchemaType {
    fn spawn<'w>(&self, world: &'w mut World) -> EntityMut<'w>;
}
