use std::marker::PhantomData;

use bevy::prelude::{Entity, Resource, Vec3};
use bevy::utils::HashMap;
use kd_tree::{KdPoint, KdTree};

use crate::plugin::SpatialPlugin;

#[derive(Resource)]
pub struct KDTreeAccess<TComp, KDItem>
where
    KDItem: KdPoint,
{
    pub tree: KdTree<KDItem>,

    /// Internal map from entity to the corresponding last position
    /// (used to make sure that removal logic is correct, and to check for significant moves)
    pub last_pos_map: HashMap<Entity, Vec3>,

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
            last_pos_map: Default::default(),
            component_type: PhantomData,
        }
    }
}
