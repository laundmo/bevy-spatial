use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_prototype_debug_lines::DebugLines;
use num_traits::{Bounded, Num, Signed};

use crate::resources_components::TimestepElapsed;

pub fn run_if_elapsed<TComp>(
    mut elapsed: ResMut<TimestepElapsed<TComp>>,
    time: Res<Time>,
) -> ShouldRun
where
    TComp: Component,
{
    if elapsed.tick(time.delta()).finished() {
        elapsed.reset();
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

/// Empty trait for numbers. A copy of R*-tree Scalar.
pub trait Scalar: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}

impl<S> Scalar for S where S: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}

/// Empty trait for the Unit generic - as that could either be Vec2 or Vec3.
pub trait Vec:
    Clone + PartialEq + Debug + Default + Add<Output = Self> + Sub<Output = Self>
{
}

/// Blanket Vec impl for everything that matches the where clause.
impl<T> Vec for T where
    T: Clone + PartialEq + Debug + Default + Add<Output = Self> + Sub<Output = Self>
{
}

pub trait AABB
where
    Self: Sized,
{
    /// Type of the glam Vec used in this AABB
    type VecType: Vec;
    /// Type of the scalars used for distances etc. Should match the VecType's numerical type.
    type ScalarType: Scalar;

    /// Center point of the AABB
    fn point(&self) -> Self::VecType;
    /// Area of the AABB
    fn area(&self) -> Self::ScalarType;
    /// Half-extents of this AABB
    fn half_extents(&self) -> Self::VecType;
    /// Upper left corner of the AABB
    fn top_left(&self) -> Self::VecType {
        self.point() + self.half_extents()
    }
    /// Lower right corner of the AABB
    fn bottom_right(&self) -> Self::VecType {
        self.point() - self.half_extents()
    }
    /// Squared distance to another AABB (based on `AABB.point()`)
    //TODO: how to inter-doc ref?
    fn distance_squared(&self, other: &Self) -> Self::ScalarType;
    /// Whether the AABB fully contains another AABB
    fn contains(&self, other: &Self) -> bool;
    /// The AABB created by the overlap.
    fn overlap(&self, other: &Self) -> Option<Self>;
}

pub trait AABBextras2d {
    /// Array containing each corner of the Rect
    fn verts(&self) -> [Vec2; 4];

    /// Draw the edges of the AABB if debug is enabled.
    #[cfg(feature = "debug")]
    fn debug_draw_line(&self, lines: &mut DebugLines, color: Color);
}

pub trait AABBextras3d {
    /// Array containing each corner of the Box
    fn verts(&self) -> [Vec3; 8];

    /// Draw the edges of the AABB if debug is enabled.
    #[cfg(feature = "debug")]
    fn debug_draw_line(&self, lines: &mut DebugLines, color: Color);
}
