pub mod common;
mod rtree2d;
mod rtree3d;

pub use rstar::{DefaultParams, RStarInsertionStrategy, RTreeParams};

use crate::plugin::SpatialPlugin;

pub use self::{rtree2d::RTreeAccess2D, rtree3d::RTreeAccess3D};

/// 2D R*-Tree plugin. Can optionally be passed R*-Tree params as a second argument.
/// See [rstar::RTreeParams] for details
pub type RTreePlugin2D<TComp, Params = DefaultParams> =
    SpatialPlugin<TComp, RTreeAccess2D<TComp, Params>>;

/// 3D R*-Tree plugin. Can optionally be passed R*-Tree params as a second argument.
/// See [rstar::RTreeParams] for details
pub type RTreePlugin3D<TComp, Params = DefaultParams> =
    SpatialPlugin<TComp, RTreeAccess3D<TComp, Params>>;
