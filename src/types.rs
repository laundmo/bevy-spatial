use bevy::prelude::*;

pub trait EntityPoint: PartialEq + Send + Sync + 'static {
    type Unit;

    fn from_vec3(src: Vec3, entity: Entity) -> Self;
    fn vec(&self) -> Self::Unit;
    fn entity(&self) -> Entity;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityPoint2D {
    pub vec: Vec2,
    pub entity: Entity,
}

impl EntityPoint for EntityPoint2D {
    type Unit = Vec2;

    fn from_vec3(src: Vec3, entity: Entity) -> Self {
        Self {
            vec: src.truncate(),
            entity,
        }
    }

    fn vec(&self) -> Self::Unit {
        self.vec
    }

    fn entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityPoint3D {
    pub vec: Vec3,
    pub entity: Entity,
}

impl EntityPoint for EntityPoint3D {
    type Unit = Vec3;

    fn from_vec3(src: Vec3, entity: Entity) -> Self {
        Self { vec: src, entity }
    }

    fn vec(&self) -> Self::Unit {
        self.vec
    }

    fn entity(&self) -> Entity {
        self.entity
    }
}
