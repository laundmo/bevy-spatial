use bevy::prelude::{Component, Entity};
use num_traits::{Bounded, Num, Signed};
use std::{fmt::Debug, marker::PhantomData};
use typenum::Unsigned;

use crate::TComp;
pub trait Scalar: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}
impl<T> Scalar for T where T: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}

// Matches closely what rstar and kdtree use
pub trait SpatialPoint: Copy + Clone + PartialEq + Debug {
    /// The Scalar type of a vector, example: f32, f64
    type Scalar: Scalar;

    /// The vector type itself, for example [`crate::point::SVec3`]
    type Vec;

    /// The dimension of this vector, like [`typenum::U2`] [`typenum::U3`]
    type Dimension: Unsigned;

    /// `nth` always smaller than [`Self::Dimension`].
    fn at(&self, nth: usize) -> Self::Scalar;

    /// Get the squared distance of this point to another point of the same type.
    fn distance_squared(&self, other: &Self) -> Self::Scalar;

    /// Get the minimum between this and another point
    fn min_point(&self, other: &Self) -> Self::Vec;

    /// Get the maximum between this and another point
    fn max_point(&self, other: &Self) -> Self::Vec;

    fn entity(&self) -> Option<Entity>;

    fn vec(&self) -> Self::Vec;
}

pub trait IntoSpatialPoint
where
    Self: Sized + Copy,
{
    type Point: SpatialPoint + From<(Entity, Self)> + Copy;
    fn into_spatial_point(self, e: Entity) -> Self::Point
    where
        Self::Point: From<(Entity, Self)>,
    {
        (e, self).into()
    }
}

macro_rules! impl_spatial_point {
    ($pointname:ident, $bvec:ty, $unit:ty, $dim:ty, $diml:literal) => {
        /// Newtype over bevy/glam vectors, needed to allow implementing foreign spatial datastructure traits.
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $pointname {
            pub vec: $bvec,
            pub entity: Option<Entity>,
        }

        impl $pointname {
            fn new(vec: $bvec, entity: Entity) -> Self {
                $pointname {
                    vec,
                    entity: Some(entity),
                }
            }

            fn from_vec(vec: $bvec) -> Self {
                $pointname { vec, entity: None }
            }
        }

        impl SpatialPoint for $pointname {
            type Scalar = $unit;
            type Vec = $bvec;
            type Dimension = $dim;

            #[inline]
            fn at(&self, nth: usize) -> Self::Scalar {
                self.vec[nth]
            }

            #[inline]
            fn distance_squared(&self, other: &Self) -> Self::Scalar {
                self.vec.distance_squared(other.vec)
            }

            #[inline]
            fn min_point(&self, other: &Self) -> Self::Vec {
                self.vec.min(other.vec)
            }

            #[inline]
            fn max_point(&self, other: &Self) -> Self::Vec {
                self.vec.max(other.vec)
            }

            #[inline]
            fn entity(&self) -> Option<Entity> {
                self.entity
            }

            #[inline]
            fn vec(&self) -> Self::Vec {
                self.vec
            }
        }

        impl From<(Entity, $bvec)> for $pointname {
            fn from(value: (Entity, $bvec)) -> Self {
                $pointname::new(value.1, value.0)
            }
        }

        impl From<($bvec, Entity)> for $pointname {
            fn from(value: ($bvec, Entity)) -> Self {
                $pointname::new(value.0, value.1)
            }
        }

        impl From<$bvec> for $pointname {
            fn from(value: $bvec) -> Self {
                $pointname::from_vec(value)
            }
        }

        impl IntoSpatialPoint for $bvec {
            type Point = $pointname;
        }
    };
}

impl_spatial_point!(Point2, bevy::math::Vec2, f32, typenum::consts::U2, 2);
impl_spatial_point!(Point3, bevy::math::Vec3, f32, typenum::consts::U3, 3);
impl_spatial_point!(Point3A, bevy::math::Vec3A, f32, typenum::consts::U3, 3);
impl_spatial_point!(PointD2, bevy::math::DVec2, f64, typenum::consts::U2, 2);
impl_spatial_point!(PointD3, bevy::math::DVec3, f64, typenum::consts::U3, 3);

#[derive(Copy, Clone, Debug, Component)]
pub struct SpatialTracker<Comp: TComp, P: IntoSpatialPoint> {
    c: PhantomData<Comp>,
    pub coord: P,
}
