//! A bevy plugin to track your entities in spatial indexes and query them.
//!
//! Quickstart using the `kdtree` feature:
//! ```rust
//! # use bevy::prelude::*;
//! use bevy_spatial::{KDTreeAccess2D, KDTreePlugin2D, SpatialAccess};
//!
//! #[derive(Component)]
//! struct TrackedByKDTree;
//!
//! fn main() {
//!    App::new()
//!        .add_plugin(KDTreePlugin2D::<TrackedByKDTree> { ..default() })
//!        .add_system(use_neighbour);
//!    // ...
//! }
//!
//! type NNTree = KDTreeAccess2D<TrackedByKDTree>; // type alias for later
//!
//! fn use_neighbour(tree: Res<NNTree>){
//!     if let Some((pos, entity)) = tree.nearest_neighbour(Vec3::ZERO) {
//!         // pos: Vec3
//!         // do something with the nearest entity here
//!     }
//! }
//! ```
//!
//! For more details see [Examples](https://github.com/laundmo/bevy-spatial/tree/main/examples)

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

mod aabb_impls;
mod common;
mod plugin;
mod resources_components;
mod spatial_access;

pub use self::{
    aabb_impls::{Cube, Point2d, Point3d, RectAABB},
    common::AABB,
    plugin::SpatialPlugin,
    spatial_access::SpatialAccess,
};

#[cfg(feature = "debug")]
mod debug_aabb;
#[cfg(feature = "debug")]
mod debug_draw_utils;
#[cfg(feature = "debug")]
pub use self::debug_aabb::DebugAABB;

#[cfg(feature = "kdtree")]
mod kdtree;
#[cfg(feature = "kdtree")]
pub use self::kdtree::{KDTreeAccess2D, KDTreePlugin2D};

#[cfg(feature = "rstar")]
mod rtree;

#[cfg(feature = "rstar")]
pub use self::rtree::{
    DefaultParams, EfficientInsertParams, RTreeAccess2D, RTreeAccess3D, RTreePlugin2D,
    RTreePlugin3D,
};
