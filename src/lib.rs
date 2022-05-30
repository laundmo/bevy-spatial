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

mod common;
#[cfg(feature = "kdtree")]
mod kdtree;
mod plugin;
mod resources_components;
#[cfg(feature = "rstar")]
mod rtree;
mod spatial_access;

pub use self::{
    common::{EntityPoint, EntityPoint2D, EntityPoint3D},
    plugin::SpatialPlugin,
    spatial_access::SpatialAccess,
};

#[cfg(feature = "kdtree")]
pub use self::kdtree::{KDTreeAccess2D, KDTreePlugin2D};

#[cfg(feature = "rstar")]
pub use self::rtree::{
    DefaultParams, EfficientInsertParams, RTreeAccess2D, RTreeAccess3D, RTreePlugin2D,
    RTreePlugin3D,
};
