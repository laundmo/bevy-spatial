pub mod common;
mod rtree2d;
mod rtree3d;

pub use rstar::{DefaultParams, RStarInsertionStrategy, RTreeParams};

use crate::plugin::SpatialPlugin;

pub use self::{rtree2d::RTreeAccess2D, rtree3d::RTreeAccess3D};

/// 2D R*-Tree plugin. Can optionally be passed R*-Tree params as a second argument.
/// See the docs here: https://docs.rs/rstar/latest/rstar/trait.RTreeParams.html
pub type RTreePlugin2D<TComp, Params = EfficientInsertParams> =
    SpatialPlugin<TComp, RTreeAccess2D<TComp, Params>>;
pub type RTreePlugin3D<TComp, Params = EfficientInsertParams> =
    SpatialPlugin<TComp, RTreeAccess3D<TComp, Params>>;

pub struct EfficientInsertParams;

impl RTreeParams for EfficientInsertParams {
    const MIN_SIZE: usize = 2;
    const MAX_SIZE: usize = 1000;
    const REINSERTION_COUNT: usize = 1;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}
