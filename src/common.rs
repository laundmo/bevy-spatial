use bevy::prelude::*;

use crate::resources_components::TimestepElapsed;

// TODO: I can't get this to work
pub fn run_if_elapsed<TComp>(mut elapsed: ResMut<TimestepElapsed<TComp>>, time: Res<Time>) -> bool
where
    TComp: Component,
{
    if elapsed.tick(time.delta()).just_finished() {
        elapsed.reset();
        true
    } else {
        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityPoint<Unit>
where
    Unit: PartialEq,
{
    pub vec: Unit,
    pub entity: Entity,
}

pub type EntityPoint2D = EntityPoint<Vec2>;

pub type EntityPoint3D = EntityPoint<Vec3>;

impl<Unit> From<(Unit, Entity)> for EntityPoint<Unit>
where
    Unit: PartialEq,
{
    fn from(thing: (Unit, Entity)) -> Self {
        EntityPoint {
            vec: thing.0,
            entity: thing.1,
        }
    }
}

impl<Unit> From<&(Unit, Entity)> for EntityPoint<Unit>
where
    Unit: PartialEq + Copy,
{
    fn from(thing: &(Unit, Entity)) -> Self {
        EntityPoint {
            vec: thing.0,
            entity: thing.1,
        }
    }
}

impl<Unit> From<Entity> for EntityPoint<Unit>
where
    Unit: PartialEq + Default,
{
    fn from(entity: Entity) -> Self {
        EntityPoint {
            vec: Unit::default(),
            entity,
        }
    }
}

// truncating Vec3 to EntityPoint2D

// reference
impl From<&(Vec3, Entity)> for EntityPoint2D {
    fn from(thing: &(Vec3, Entity)) -> Self {
        EntityPoint2D {
            vec: thing.0.truncate(),
            entity: thing.1,
        }
    }
}

impl From<&(Entity, Vec3)> for EntityPoint2D {
    fn from(thing: &(Entity, Vec3)) -> Self {
        EntityPoint2D {
            vec: thing.1.truncate(),
            entity: thing.0,
        }
    }
}

// value
impl From<(Vec3, Entity)> for EntityPoint2D {
    fn from(thing: (Vec3, Entity)) -> Self {
        EntityPoint2D {
            vec: thing.0.truncate(),
            entity: thing.1,
        }
    }
}

impl From<(Entity, Vec3)> for EntityPoint2D {
    fn from(thing: (Entity, Vec3)) -> Self {
        EntityPoint2D {
            vec: thing.1.truncate(),
            entity: thing.0,
        }
    }
}

// the compiler wont allow these to be generic??

impl From<(&Transform, Entity)> for EntityPoint2D {
    fn from(thing: (&Transform, Entity)) -> Self {
        Self::from((thing.0.translation, thing.1))
    }
}

impl From<(Entity, &Transform)> for EntityPoint2D {
    fn from(thing: (Entity, &Transform)) -> Self {
        Self::from((thing.1.translation, thing.0))
    }
}

impl From<(&Transform, Entity)> for EntityPoint3D {
    fn from(thing: (&Transform, Entity)) -> Self {
        Self::from((thing.0.translation, thing.1))
    }
}

impl From<(Entity, &Transform)> for EntityPoint3D {
    fn from(thing: (Entity, &Transform)) -> Self {
        Self::from((thing.1.translation, thing.0))
    }
}
