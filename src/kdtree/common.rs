use std::marker::PhantomData;

use bevy::prelude::Resource;
use kd_tree::{KdPoint, KdTree};

use crate::plugin::SpatialPlugin;

#[derive(Resource)]
pub struct KDTreeAccess<TComp, KDItem>
where
    KDItem: KdPoint,
{
    pub tree: KdTree<KDItem>,
    pub component_type: PhantomData<TComp>,
}

impl<TComp, KDItem> From<SpatialPlugin<TComp, KDTreeAccess<TComp, KDItem>>>
    for KDTreeAccess<TComp, KDItem>
where
    KDItem: KdPoint,
    <KDItem as KdPoint>::Scalar: num_traits::Float,
{
    fn from(_: SpatialPlugin<TComp, KDTreeAccess<TComp, KDItem>>) -> Self {
        let tree: KdTree<KDItem> = KdTree::<KDItem>::build_by_ordered_float(vec![]);

        KDTreeAccess {
            tree,
            component_type: PhantomData,
        }
    }
}
