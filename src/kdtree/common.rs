use std::marker::PhantomData;

use kd_tree::{KdPoint, KdTree};

use crate::plugin::SpatialPlugin;

pub struct KDTreeAccess<TComp, KDItem>
where
    KDItem: KdPoint,
{
    pub tree: KdTree<KDItem>,
    pub min_moved: f32,
    pub component_type: PhantomData<TComp>,
}

impl<TComp, KDItem> From<SpatialPlugin<TComp, KDTreeAccess<TComp, KDItem>>>
    for KDTreeAccess<TComp, KDItem>
where
    KDItem: KdPoint,
    <KDItem as KdPoint>::Scalar: num_traits::Float,
{
    fn from(plugin: SpatialPlugin<TComp, KDTreeAccess<TComp, KDItem>>) -> Self {
        let tree: KdTree<KDItem> = KdTree::<KDItem>::build_by_ordered_float(vec![]);

        KDTreeAccess {
            tree,
            min_moved: plugin.min_moved,
            component_type: PhantomData,
        }
    }
}
