#![warn(missing_docs)]
#![deny(clippy::pedantic)]

//! A bevy plugin to track your entities in spatial indices and query them.
//!
//! Quickstart using the `kdtree` feature:
//! ```
//! use bevy_spatial::{AutomaticUpdate, KDTree3, TransformMode, SpatialAccess};
//!
//! #[derive(Component, Default)]
//! struct TrackedByKDTree;
//!
//! fn main() {
//!    App::new()
//!        .add_plugin(AutomaticUpdate::<TrackedByKDTree>::new()
//!             .with_frequency(Duration::from_secs_f32(0.3))
//!             .with_transform(TransformMode::GlobalTransform))
//!        .add_system(use_neighbour);
//!    // ...
//! }
//!
//! type NNTree = KDTree3<TrackedByKDTree>; // type alias for later
//!
//! // spawn some entities with the TrackedByKDTree component
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

pub mod point;
mod spatial_access;
pub use self::spatial_access::{SpatialAccess, SpatialAABBAccess};

use bevy::prelude::Component;
mod timestep;
pub use self::timestep::TimestepLength;

pub mod kdtree;

mod plugin;
pub use plugin::{SpatialStructure, *};

mod automatic_systems;
pub use automatic_systems::TransformMode;

/// automatically implemented trait for all components which can be used as markers for automatic updates?
pub trait TComp: Component + Send + Sync + 'static {}
impl<T> TComp for T where T: Component + Send + Sync + 'static {}
