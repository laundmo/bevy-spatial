use std::fmt::Debug;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
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
pub trait Vec: Clone + PartialEq + Debug + Default {}

/// Blanket Vec impl for everything that matches the where clause.
impl<T> Vec for T where T: Clone + PartialEq + Debug + Default {}

pub trait AABB {
    /// Type of the glam Vec used in this AABB
    type VecType: Vec;
    /// Type of the scalars used for distances etc. Should match the VecType's numerical type.
    type ScalarType: Scalar;

    /// Center point of the AABB
    fn point(&self) -> Self::VecType;
    /// Squared distance to another AABB (based on `AABB.point()`)
    //TODO: how to ref?
    fn distance_squared(&self, other: &Self) -> Self::ScalarType;
    /// Upper left corner of the AABB
    fn upper(&self) -> Self::VecType;
    /// Lower right corner of the AABB
    fn lower(&self) -> Self::VecType;
    /// Whether the AABB fully contains another AABB
    fn contains(&self, other: &Self) -> bool;
    /// Whether the AABB Overlaps with another AABB
    fn overlaps(&self, other: &Self) -> bool;
    /// The overlap area size between this and another AABB
    fn overlap_area(&self, other: &Self) -> Self::ScalarType;
}
