//! A bevy plugin to track your entities in spatial indices and query them.
//!
//! Quickstart using the `kdtree` feature:
//! ```rust
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
//! // spawn some entities with the TrackedByKDTree component
//!
//! fn use_neighbour(tree: Res<NNTree>){
//!     if let Some((pos, entity)) = tree.nearest_neighbour(Vec2::ZERO) {
//!         // pos: Vec3
//!         // do something with the nearest entity here
//!     }
//! }
//! ```
//!
//! For more details see [Examples](https://github.com/laundmo/bevy-spatial/tree/main/examples)

// mod aabb;
// mod datacontainer;
mod point;
mod spatial_access;
pub use self::spatial_access::SpatialAccess;
// mod test_plugin;

use bevy::prelude::Component;
// pub use test_plugin::TestPlugin;
mod timestep;
pub use self::timestep::TimestepLength;
// mod types;

mod kdtree;
pub use self::kdtree::*;

mod plugin;
pub use self::plugin::Spatial;

mod automatic_systems;
// mod resources_components;
// #[cfg(feature = "rstar")]
// mod rtree;
// mod spatial_access;

// pub use self::{
//     // common::{EntityPoint, EntityPoint2D, EntityPoint3D},
//     plugin::SpatialPlugin,
//     spatial_access::SpatialAccess,
//     timestep::TimestepElapsed,
// };

// #[cfg(feature = "rstar")]
// pub use self::rtree::{DefaultParams, RTreeAccess2D, RTreeAccess3D, RTreePlugin2D, RTreePlugin3D};

// Trait bound aliases
pub trait TComp: Component + Default + Send + Sync + 'static {}
impl<T> TComp for T where T: Component + Default + Send + Sync + 'static {}
