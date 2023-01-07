use bevy::{
    math::{DVec2, DVec3, Vec3A},
    prelude::*,
};
use num_traits::{Bounded, Num, Signed};
use std::fmt::Debug;
pub trait Unit: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}
impl<T> Unit for T where T: Bounded + Num + Clone + Copy + Signed + PartialOrd + Debug {}

// Closely inspired by, almost copied from, rstar
pub trait SpatialPoint: Copy + Clone + PartialEq + Debug {
    /// todo:
    type Unit: Unit;

    /// todo:
    const DIMENSIONS: usize;

    fn create(test: &[Self::Unit]) -> Self;

    /// `nth` always smaller than `Self::DIMENSIONS`.
    fn at(&self, nth: usize) -> Self::Unit;

    /// Get the squared distance of this point to another point of the same type.
    fn distance_squared(&self, other: &Self) -> Self::Unit;

    fn min_point(&self, other: &Self) -> Self;

    fn max_point(&self, other: &Self) -> Self;
}

macro_rules! impl_spatial_point_glam {
    ($vec:ident, $unit:ident, $dist_t:ident) => {
        impl SpatialPoint for $vec {
            type Unit = $unit;

            const DIMENSIONS: usize = 2;

            fn create(data: &[Self::Unit]) -> Self {
                $vec::from_slice(data)
            }

            fn at(&self, nth: usize) -> Self::Unit {
                self[nth]
            }

            fn distance_squared(&self, other: &Self) -> $dist_t {
                $vec::distance_squared(*self, *other)
            }

            fn min_point(&self, other: &Self) -> Self {
                self.min(*other)
            }

            fn max_point(&self, other: &Self) -> Self {
                self.max(*other)
            }
        }
    };
}

impl_spatial_point_glam!(Vec2, f32, f32);
impl_spatial_point_glam!(Vec3, f32, f32);
impl_spatial_point_glam!(Vec3A, f32, f32);
impl_spatial_point_glam!(DVec2, f64, f64);
impl_spatial_point_glam!(DVec3, f64, f64);
