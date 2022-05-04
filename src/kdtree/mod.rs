mod common;
mod kdtree2d;

use crate::plugin::SpatialPlugin;

pub use self::kdtree2d::kdtree::KDTreeAccess2D;

/// Plugin specific to the 2D KD-Tree.
/// If you despawn entities you will have to add this without a timestep, or check if the entity still exists.
pub type KDTreePlugin2D<TComp> = SpatialPlugin<TComp, KDTreeAccess2D<TComp>>;
