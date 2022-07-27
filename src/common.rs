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
    /// Area of the AABB
    fn area(&self) -> Self::ScalarType;
    /// Squared distance to another AABB (based on `AABB.point()`)
    //TODO: how to inter-doc ref?
    fn distance_squared(&self, other: &Self) -> Self::ScalarType;
    /// Whether the AABB fully contains another AABB
    fn contains(&self, other: &Self) -> bool;
    /// The AABB created by the overlap.
    fn overlap(&self, other: &Self) -> Option<&Self>;
}

pub trait AABBExtras2D {
    type AABBrel: AABB;
    fn corners(&self) -> [AABBExtras2D::AABBrel::VecType; 2];
}
