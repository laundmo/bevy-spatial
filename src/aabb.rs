use bevy::math::Vec3A;
use bevy::render::primitives::Aabb as BevyAabb;

use crate::point::SpatialPoint;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Aabb<P>
where
    P: SpatialPoint,
{
    lower: P,
    upper: P,
}

impl From<BevyAabb> for Aabb<Vec3A> {
    fn from(aabb: BevyAabb) -> Self {
        Self {
            lower: aabb.center + aabb.half_extents,
            upper: aabb.center - aabb.half_extents,
        }
    }
}

// impl From<> for Aabb<Vec3A> {
//     fn from(aabb: BevyAabb) -> Self {
//         Self {
//             lower: aabb.center + aabb.half_extents,
//             upper: aabb.center - aabb.half_extents,
//         }
//     }
// }
