use bevy::{
    math::{Vec2, Vec3, Vec3Swizzles},
    prelude::{Color, Entity, Transform},
};

#[cfg(feature = "debug")]
use bevy_prototype_debug_lines::DebugLines;

use crate::{
    common::{AABBextras2d, AABBextras3d, Vec},
    AABB,
};

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

    /// For point AABBs always Vec2::ZERO
    fn half_extents(&self) -> Self::VecType {
        Vec2::ZERO
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

    fn overlap(&self, other: &Self) -> Option<Self> {
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

    /// For point AABBs always Vec3::ZERO
    fn half_extents(&self) -> Self::VecType {
        Vec3::ZERO
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
    fn overlap(&self, other: &Self) -> Option<Self> {
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

pub type CubeAABB = RectBase<Vec3>;
impl CubeAABB {
    pub fn new(tr: &Transform, entity: Entity) -> Self {
        Self {
            pos: tr.translation.xyz(),
            extent: tr.scale.xyz(),
            entity: Some(entity),
        }
    }
}

fn scalar_overlap(p1: f32, extent1: f32, p2: f32, extent2: f32) -> f32 {
    let dist = (p1 - p2).abs();
    (dist - (extent1 + extent2)).abs()
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

    fn half_extents(&self) -> Self::VecType {
        self.extent / 2.0 // TODO: depends on whether bevy scale is already half
    }

    fn distance_squared(&self, other: &Self) -> Self::ScalarType {
        self.point().distance_squared(other.point())
    }

    fn contains(&self, other: &Self) -> bool {
        other.bottom_right().cmpge(self.bottom_right()).all()
            && other.top_left().cmple(self.top_left()).all()
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        // Early-exit on other containing self.
        if other.contains(&self) {
            return Some(self.clone());
        }

        let midpoint = self.point().lerp(other.point(), 0.5);

        let overlap = Self {
            pos: midpoint,
            extent: Vec2::new(
                scalar_overlap(
                    self.point().x,
                    self.half_extents().x,
                    other.point().x,
                    other.half_extents().x,
                ),
                scalar_overlap(
                    self.point().y,
                    self.half_extents().y,
                    other.point().y,
                    other.half_extents().y,
                ),
            ),
            entity: None,
        };
        return Some(overlap);
    }
}

const RECT_EDGES: [(usize, usize); 4] = [(0, 1), (1, 2), (2, 3), (3, 0)];

impl AABBextras2d for RectAABB {
    /// Vertexes of this RectAABB from upper left to lower left in clockwise direction.
    fn verts(&self) -> [Vec2; 4] {
        let x = self.half_extents().x;
        let y = self.half_extents().y;

        [
            self.point() + Vec2::new(x, y),   // upper left
            self.point() + Vec2::new(-x, y),  // upper right
            self.point() + Vec2::new(-x, -y), // lower right
            self.point() + Vec2::new(x, -y),  // lower left
        ]
    }

    #[cfg(feature = "debug")]
    fn debug_draw_line(&self, lines: &mut DebugLines, color: Color) {
        let verts = self.verts();
        for (p0, p1) in RECT_EDGES {
            lines.line_colored(verts[p0].extend(0.0), verts[p1].extend(0.0), 0.0, color);
        }
    }
}

impl From<(Entity, &Transform)> for RectAABB {
    fn from(other: (Entity, &Transform)) -> Self {
        RectAABB::new(other.1, other.0)
    }
}

impl AABB for CubeAABB {
    type VecType = Vec3;
    type ScalarType = f32;

    fn point(&self) -> Self::VecType {
        self.pos
    }

    fn half_extents(&self) -> Self::VecType {
        self.extent / 2.0 // TODO: depends on whether bevy scale is already half
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

    fn overlap(&self, other: &Self) -> Option<Self> {
        // Early-exit on other containing self.
        if other.contains(&self) {
            return Some(self.clone());
        }

        let midpoint = self.point().lerp(other.point(), 0.5);

        let overlap = Self {
            pos: midpoint,
            extent: Vec3::new(
                scalar_overlap(self.pos.x, self.extent.x, other.pos.x, other.extent.x) / 2.0,
                scalar_overlap(self.pos.y, self.extent.y, other.pos.y, other.extent.y) / 2.0,
                scalar_overlap(self.pos.z, self.extent.z, other.pos.z, other.extent.z) / 2.0,
            ),
            entity: None,
        };
        return Some(overlap);
    }
}

// Thanks heron! https://github.com/jcornaz/heron/blob/main/debug/src/shape3d_wireframe.rs
const CUBOID_EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 5),
    (1, 6),
    (2, 7),
    (3, 4),
];

impl AABBextras3d for CubeAABB {
    fn verts(&self) -> [Vec3; 8] {
        let x = self.half_extents().x;
        let y = self.half_extents().y;
        let z = self.half_extents().z;

        [
            self.point() + Vec3::new(x, y, z),
            self.point() + Vec3::new(-x, y, z),
            self.point() + Vec3::new(-x, y, -z),
            self.point() + Vec3::new(x, y, -z),
            self.point() + Vec3::new(x, -y, -z),
            self.point() + Vec3::new(x, -y, z),
            self.point() + Vec3::new(-x, -y, z),
            self.point() + Vec3::new(-x, -y, -z),
        ]
    }

    #[cfg(feature = "debug")]
    fn debug_draw_line(&self, lines: &mut DebugLines, color: Color) {
        let verts = self.verts();
        for &(p0, p1) in &CUBOID_EDGES {
            lines.line_colored(verts[p0], verts[p1], 0.0, color);
        }
    }
}
