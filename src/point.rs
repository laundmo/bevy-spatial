//! The different point Traits and Types used by ``bevy_spatial``
//!
//! - [`Scalar`] is a Trait based on [`num_traits`] which is implemented for all numeric types used in Points.
//! - [`SpatialPoint`] is a Trait that represents a point in space and the Entity it was created from.
//!   It defines some, common methods needed while working with these points in different spatial datastructures.
//! - [`IntoSpatialPoint`] is a Trait which is implemented for vector coordinate types for which a corresponding Point type exists.
//!   Needs a [`Entity`] to include in the Point type.
//! - [`VecFromTransform`] and [`VecFromGlobalTransform`] used to extract the translation from the corresponding Transform.
//!   Used for automatically updating the spatial datastructure.

use bevy::{math::Vec3A, prelude::*};
use num_traits::{Bounded, Num, Signed};
use std::fmt::Debug;
use typenum::Unsigned;

/// Trait implemented for all numeric types used in Points.
pub trait Scalar: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}
impl<T> Scalar for T where T: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}

/// Represents a point in space and the Entity it was created from.
///
/// Implements a bunch of common methods needed while working with these points in different spatial datastructures.
#[allow(clippy::module_name_repetitions)]
pub trait SpatialPoint: Copy + Clone + PartialEq + Debug {
    /// The Scalar type of a vector, example: [`f32`], [`f64`]
    type Scalar: Scalar;

    /// The vector type itself, for example [`Vec3`](bevy::prelude::Vec3)
    type Vec: Send + Sync + IntoSpatialPoint;

    /// The dimension of this vector, like [`typenum::U2`] [`typenum::U3`]
    type Dimension: Unsigned;

    /// Get the value at this index.
    /// Used for datastructure specific implementations.
    ///
    /// `nth` is always smaller than [`Self::Dimension`].
    fn at(&self, nth: usize) -> Self::Scalar;

    /// Get the squared distance of this point to another point of the same type.
    fn distance_squared(&self, other: &Self) -> Self::Scalar;

    /// Get the elementwise minimum between this and another point
    fn min_point(&self, other: &Self) -> Self::Vec;

    /// Get the elementwise maximum between this and another point
    fn max_point(&self, other: &Self) -> Self::Vec;

    /// Get the Entity associated with this point.
    fn entity(&self) -> Option<Entity>;

    /// Get a this points vector.
    fn vec(&self) -> Self::Vec;
}

/// Trait implemented for vector coordinate types for which a corresponding Point type exists.
/// Used to convert from the vector coordinate types to the corresponding Point type by providing a Entity
#[allow(clippy::module_name_repetitions)]
pub trait IntoSpatialPoint: Send + Sync + Sized + Copy {
    /// The resulting point type, for example [`Point3`]
    type Point: SpatialPoint + From<(Entity, Self)> + Copy;

    /// Converts from the implementing type to the point type with its Entity filled.
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
            /// The vector of this Point
            pub vec: $bvec,
            /// The Entity associated with this Point
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

/// Helper trait for extracting the translation of a [`Transform`] to a specific vector type
/// Used for automatically updating the spatial datastructure.
pub trait VecFromTransform: IntoSpatialPoint {
    /// Create this vector type from a [`Transform`]
    fn from_transform(t: &Transform) -> Self;
}

impl VecFromTransform for Vec2 {
    fn from_transform(t: &Transform) -> Self {
        t.translation.truncate()
    }
}
impl VecFromTransform for Vec3 {
    fn from_transform(t: &Transform) -> Self {
        t.translation
    }
}
impl VecFromTransform for Vec3A {
    fn from_transform(t: &Transform) -> Self {
        t.translation.into()
    }
}

/// Helper trait for extracting the translation of a [`GlobalTransform`] to a specific vector type
/// Used for automatically updating the spatial datastructure.

pub trait VecFromGlobalTransform: IntoSpatialPoint {
    /// Create this vector type from a [`GlobalTransform`]
    fn from_transform(t: &GlobalTransform) -> Self;
}

impl VecFromGlobalTransform for Vec2 {
    fn from_transform(t: &GlobalTransform) -> Self {
        t.translation().truncate()
    }
}
impl VecFromGlobalTransform for Vec3 {
    fn from_transform(t: &GlobalTransform) -> Self {
        t.translation()
    }
}
impl VecFromGlobalTransform for Vec3A {
    fn from_transform(t: &GlobalTransform) -> Self {
        t.translation().into()
    }
}
