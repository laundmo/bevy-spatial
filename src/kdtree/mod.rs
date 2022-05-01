mod common;
mod kdtree2d;

use crate::plugin::SpatialPlugin;

pub use self::kdtree2d::kdtree::KDTreeAccess2D;

pub type KDTreePlugin2D<TComp> = SpatialPlugin<TComp, KDTreeAccess2D<TComp>>;
