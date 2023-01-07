use bevy::{math::Vec3A, prelude::*};
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

    fn min(a: Self::Unit, b: Self::Unit) -> Self::Unit;

    fn max(a: Self::Unit, b: Self::Unit) -> Self::Unit;

    fn create(test: &[Self::Unit]) -> Self;

    /// `nth` always smaller than `Self::DIMENSIONS`.
    fn at(&self, nth: usize) -> Self::Unit;

    /// Get the squared distance of this point to another point of the same type.
    fn distance_squared(&self, other: &Self) -> Self::Unit;

    fn min_point(&self, other: &Self) -> Self {
        Self::create(
            &(0..Self::DIMENSIONS)
                .into_iter()
                .map(|nth| Self::min(self.at(nth), other.at(nth)))
                .collect::<Vec<_>>(),
        )
    }

    fn max_point(&self, other: &Self) -> Self {
        Self::create(
            &(0..Self::DIMENSIONS)
                .into_iter()
                .map(|nth| Self::max(self.at(nth), other.at(nth)))
                .collect::<Vec<_>>(),
        )
    }
}

impl SpatialPoint for Vec2 {
    type Unit = f32;

    const DIMENSIONS: usize = 2;

    fn create(data: &[Self::Unit]) -> Self {
        Vec2::from_slice(data)
    }

    fn at(&self, nth: usize) -> Self::Unit {
        self[nth]
    }

    #[inline]
    fn min(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        f32::min(a, b)
    }
    #[inline]
    fn max(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        f32::max(a, b)
    }

    fn distance_squared(&self, other: &Self) -> Self::Unit {
        Vec2::distance_squared(*self, *other)
    }
}

impl SpatialPoint for Vec3 {
    type Unit = f32;

    const DIMENSIONS: usize = 3;

    fn create(data: &[Self::Unit]) -> Self {
        Vec3::from_slice(data)
    }

    fn at(&self, nth: usize) -> Self::Unit {
        self[nth]
    }

    #[inline]
    fn min(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        f32::min(a, b)
    }

    #[inline]
    fn max(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        f32::max(a, b)
    }

    fn distance_squared(&self, other: &Self) -> Self::Unit {
        Vec3::distance_squared(*self, *other)
    }
}

impl SpatialPoint for Vec3A {
    type Unit = f32;

    const DIMENSIONS: usize = 3;

    fn create(data: &[Self::Unit]) -> Self {
        Vec3A::from_slice(data)
    }

    fn at(&self, nth: usize) -> Self::Unit {
        self[nth]
    }

    #[inline]
    fn min(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        f32::min(a, b)
    }

    #[inline]
    fn max(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        f32::max(a, b)
    }

    fn distance_squared(&self, other: &Self) -> Self::Unit {
        Vec3A::distance_squared(*self, *other)
    }
}
