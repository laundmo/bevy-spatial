//! A bevy plugin to track your entities in spatial indexes and query them.
//!
//! Qucikstart using the `kdtree` feature:
//! ```rust
//! use bevy_spatial::{KDTreeAccess2D, KDTreePlugin2D, SpatialAccess};
//!
//! #[derive(Component)]
//! struct TrackedByKDTree;
//!
//! fn main() {
//!    App::new()
//!        .add_plugin(KDTreePlugin2D::<TrackedByKDTree> { ..default() })
//!    // ...
//! }
//!
//! type NNTree = KDTreeAccess2D<TrackedByKDTree>; // type alias for later
//!
//! fn (tree: Res<NNTree>){
//!     if let Some((pos, entity)) = treeaccess.nearest_neighbour(Vec2::ZERO) {
//!         // pos: Vec3
//!         // do something with the nearest entity here
//!     }
//! }
//! ```
//!
//! For more details see [Examples](https://github.com/laundmo/bevy-spatial/tree/main/examples)

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
    DefaultParams, MovingObjectsParams, RTreeAccess2D, RTreeAccess3D, RTreePlugin2D, RTreePlugin3D,
};
