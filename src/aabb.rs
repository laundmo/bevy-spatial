use bevy::render::primitives::Aabb as BevyAabb;

use crate::point::{SVec3A, SpatialPoint};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Aabb<P>
where
    P: SpatialPoint,
{
    lower: P,
    upper: P,
}

impl From<BevyAabb> for Aabb<SVec3A> {
    fn from(aabb: BevyAabb) -> Self {
        Self {
            lower: SVec3A(aabb.center + aabb.half_extents),
            upper: SVec3A(aabb.center - aabb.half_extents),
        }
    }
}
