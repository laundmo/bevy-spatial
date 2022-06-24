use bevy::{
    math::{Vec2, Vec3, Vec3Swizzles},
    prelude::{Entity, Transform},
};

use crate::{common::Vec, AABB};

#[derive(Copy, Clone, Debug)]
pub struct Point<Unit>
where
    Unit: Vec,
{
    /// Center position of the area.
    pub pos: Unit,
    /// The entity which this area belongs to.
    pub entity: Entity,
}

pub type Point2d = Point<Vec2>;
impl Point2d {
    pub fn new(tr: &Transform, entity: &Entity) -> Self {
        Self {
            pos: tr.translation.xy(),
            entity: *entity,
        }
    }
}
pub type Point3d = Point<Vec3>;
impl Point3d {
    pub fn new(tr: &Transform, entity: &Entity) -> Self {
        Self {
            pos: tr.translation.xyz(),
            entity: *entity,
        }
    }
}
impl AABB for Point<Vec2> {
    type VecType = Vec2;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    // always false since this aabb is a point
    fn contains(&self, _other: &Self) -> bool {
        false
    }

    fn lower(&self) -> Self::VecType {
        self.point()
    }

    fn upper(&self) -> Self::VecType {
        self.point()
    }

    fn overlap_area(&self, _other: &Self) -> Self::ScalarType {
        0.0
    }

    fn overlaps(&self, _other: &Self) -> bool {
        false
    }
}

impl AABB for Point<Vec3> {
    type VecType = Vec3;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    // always false since this aabb is a point
    fn contains(&self, _other: &Self) -> bool {
        false
    }

    fn lower(&self) -> Self::VecType {
        self.point()
    }

    fn upper(&self) -> Self::VecType {
        self.point()
    }

    fn overlap_area(&self, _other: &Self) -> Self::ScalarType {
        0.0
    }

    fn overlaps(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RectBase<Unit>
where
    Unit: Vec,
{
    /// Center position of the area.
    pub pos: Unit,
    /// Extent (diameter in all directions)
    pub extent: Unit,
    /// The entity which this area belongs to.
    pub entity: Entity,
}

pub type Rect = RectBase<Vec2>;
impl Rect {
    pub fn new(tr: &Transform, entity: Entity) -> Self {
        Self {
            pos: tr.translation.xy(),
            extent: tr.scale.xy(),
            entity,
        }
    }
}

pub type Cube = RectBase<Vec3>;
impl Cube {
    pub fn new(tr: &Transform, entity: Entity) -> Self {
        Self {
            pos: tr.translation.xyz(),
            extent: tr.scale.xyz(),
            entity,
        }
    }
}

impl AABB for Rect {
    type VecType = Vec2;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    fn contains(&self, other: &Self) -> bool {
        other.lower().cmpge(self.lower()).all() && other.upper().cmple(self.upper()).all()
    }

    fn lower(&self) -> Self::VecType {
        self.point() - (self.extent / 2.0)
    }

    fn upper(&self) -> Self::VecType {
        self.point() + (self.extent / 2.0)
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.lower().cmpge(self.lower()).any() || other.upper().cmple(self.upper()).any()
    }

    fn overlap_area(&self, other: &Self) -> Self::ScalarType {
        if !self.overlaps(other) {
            return 0.0;
        }

        let umin = self.upper().min(other.upper());

        let lmax = self.lower().max(other.lower());

        let xd = umin.x - lmax.x;
        let yd = umin.y - lmax.y;
        return xd * yd;
    }
}

impl AABB for Cube {
    type VecType = Vec3;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    fn contains(&self, other: &Self) -> bool {
        other.lower().cmpge(self.lower()).all() && other.upper().cmple(self.upper()).all()
    }

    fn lower(&self) -> Self::VecType {
        self.point() - (self.extent / 2.0)
    }

    fn upper(&self) -> Self::VecType {
        self.point() + (self.extent / 2.0)
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.lower().cmpge(self.lower()).any() || other.upper().cmple(self.upper()).any()
    }

    fn overlap_area(&self, other: &Self) -> Self::ScalarType {
        if !self.overlaps(other) {
            return 0.0;
        }

        let umin = self.upper().min(other.upper());

        let lmax = self.lower().max(other.lower());

        let xd = umin.x - lmax.x;
        let yd = umin.y - lmax.y;
        let zd = umin.z - lmax.z;
        return xd * yd * zd;
    }
}
