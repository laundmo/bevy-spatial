pub mod common;
mod rtree2d;
mod rtree3d;

pub use rstar::{DefaultParams, RStarInsertionStrategy, RTreeParams};

pub use self::{
    rtree2d::{plugin::RTreePlugin2D, rtree_obj::TreeAccess2D},
    rtree3d::{plugin::RTreePlugin3D, rtree_obj::TreeAccess3D},
};

pub struct MovingObjectsParams;

impl RTreeParams for MovingObjectsParams {
    const MIN_SIZE: usize = 2;
    const MAX_SIZE: usize = 1000;
    const REINSERTION_COUNT: usize = 1;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}
