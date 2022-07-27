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
    pub entity: Option<Entity>,
}

pub type Point2d = Point<Vec2>;
impl Point2d {
    pub fn new(tr: &Transform, entity: &Entity) -> Self {
        Self {
            pos: tr.translation.xy(),
            entity: Some(*entity),
        }
    }
}
pub type Point3d = Point<Vec3>;
impl Point3d {
    pub fn new(tr: &Transform, entity: &Entity) -> Self {
        Self {
            pos: tr.translation.xyz(),
            entity: Some(*entity),
        }
    }
}
impl AABB for Point<Vec2> {
    type VecType = Vec2;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    /// For point AABBs always `0.0`
    fn area(&self) -> Self::ScalarType {
        0.
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    // always false since this aabb is a point
    fn contains(&self, _other: &Self) -> bool {
        false
    }

    fn bottom_right(&self) -> Self::VecType {
        self.point()
    }

    fn top_left(&self) -> Self::VecType {
        self.point()
    }

    fn overlap(&self, other: &Self) -> Option<&Self> {
        None
    }
}

impl AABB for Point<Vec3> {
    type VecType = Vec3;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    /// For point AABBs always `0.0`
    fn area(&self) -> Self::ScalarType {
        0.
    }

    /// Squared distance to the other.point() of another AABB
    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    /// For point AABBs always false.
    fn contains(&self, _other: &Self) -> bool {
        false
    }

    /// For point AABBs always self.point()
    fn bottom_right(&self) -> Self::VecType {
        self.point()
    }

    /// For point AABBs always self.point()
    fn top_left(&self) -> Self::VecType {
        self.point()
    }

    /// For point AABBs always None.
    fn overlap(&self, other: &Self) -> Option<&Self> {
        None
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
    pub entity: Option<Entity>,
}

pub type RectAABB = RectBase<Vec2>;
impl RectAABB {
    pub fn new(tr: &Transform, entity: Entity) -> Self {
        Self {
            pos: tr.translation.xy(),
            extent: tr.scale.xy(),
            entity: Some(entity),
        }
    }
}

pub type Cube = RectBase<Vec3>;
impl Cube {
    pub fn new(tr: &Transform, entity: Entity) -> Self {
        Self {
            pos: tr.translation.xyz(),
            extent: tr.scale.xyz(),
            entity: Some(entity),
        }
    }
}

impl AABB for RectAABB {
    type VecType = Vec2;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    fn area(&self) -> Self::ScalarType {
        self.extent.x * self.extent.y
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    fn contains(&self, other: &Self) -> bool {
        other.bottom_right().cmpge(self.bottom_right()).all()
            && other.top_left().cmple(self.top_left()).all()
    }

    fn bottom_right(&self) -> Self::VecType {
        self.point() - (self.extent / 2.0)
    }

    fn top_left(&self) -> Self::VecType {
        self.point() + (self.extent / 2.0)
    }

    fn overlap(&self, other: &Self) -> Option<&Self> {
        let candidate = Self {
            pos: self.point().lerp(other.point(), 0.5),
            extent: Vec2::new(
                self.extent.x - other.extent.x,
                self.extent.y - other.extent.y,
            ), // TODO: check if this is correct. might need some if negative = 0. check. model in desmos.
            entity: None,
        };
    }

    // fn overlaps(&self, other: &Self) -> bool {
    //     other.bottom_right().cmpge(self.bottom_right()).any()
    //         || other.top_left().cmple(self.top_left()).any()
    // }

    // fn overlap_area(&self, other: &Self) -> Self::ScalarType {
    //     if !self.overlaps(other) {
    //         return 0.0;
    //     }

    //     let umin = self.top_left().min(other.top_left());

    //     let lmax = self.bottom_right().max(other.bottom_right());

    //     let xd = umin.x - lmax.x;
    //     let yd = umin.y - lmax.y;
    //     return xd * yd;
    // }
}

impl From<(Entity, &Transform)> for RectAABB {
    fn from(other: (Entity, &Transform)) -> Self {
        RectAABB::new(other.1, other.0)
    }
}

impl AABB for Cube {
    type VecType = Vec3;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    fn area(&self) -> Self::ScalarType {
        self.extent.x * self.extent.y * self.extent.z
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    fn contains(&self, other: &Self) -> bool {
        other.bottom_right().cmpge(self.bottom_right()).all()
            && other.top_left().cmple(self.top_left()).all()
    }

    fn bottom_right(&self) -> Self::VecType {
        self.point() - (self.extent / 2.0)
    }

    fn top_left(&self) -> Self::VecType {
        self.point() + (self.extent / 2.0)
    }

    fn overlap(&self, other: &Self) -> Option<&Self> {
        todo!();
        None
    }

    // fn overlaps(&self, other: &Self) -> bool {
    //     other.bottom_right().cmpge(self.bottom_right()).any()
    //         || other.top_left().cmple(self.top_left()).any()
    // }

    // fn overlap_area(&self, other: &Self) -> Self::ScalarType {
    //     if !self.overlaps(other) {
    //         return 0.0;
    //     }

    //     let umin = self.top_left().min(other.top_left());

    //     let lmax = self.bottom_right().max(other.bottom_right());

    //     let xd = umin.x - lmax.x;
    //     let yd = umin.y - lmax.y;
    //     let zd = umin.z - lmax.z;
    //     return xd * yd * zd;
    // }
}
