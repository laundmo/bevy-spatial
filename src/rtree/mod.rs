pub mod common;
mod rtree2d;
mod rtree3d;

pub use rstar::{DefaultParams, RStarInsertionStrategy, RTreeParams};

use crate::plugin::SpatialPlugin;

pub use self::{rtree2d::RTreeAccess2D, rtree3d::RTreeAccess3D};

pub type RTreePlugin2D<TComp, Params> = SpatialPlugin<TComp, RTreeAccess2D<TComp, Params>>;
pub type RTreePlugin3D<TComp, Params> = SpatialPlugin<TComp, RTreeAccess3D<TComp, Params>>;

pub struct MovingObjectsParams;

impl RTreeParams for MovingObjectsParams {
    const MIN_SIZE: usize = 2;
    const MAX_SIZE: usize = 1000;
    const REINSERTION_COUNT: usize = 1;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}
