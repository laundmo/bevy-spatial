use std::marker::PhantomData;

use rstar::{RTree, RTreeObject, RTreeParams};

use crate::plugin::SpatialPlugin;

/// The Resource which stores the spatial datastructure.
pub struct RTreeAccess<TComp, RObj, Params>
where
    RObj: RTreeObject,
    Params: RTreeParams,
{
    pub tree: RTree<RObj, Params>,
    pub recreate_after: usize,
    pub min_moved: f32,
    pub component_type: PhantomData<TComp>,
}

impl<TComp, RObj, Params> From<SpatialPlugin<TComp, RTreeAccess<TComp, RObj, Params>>>
    for RTreeAccess<TComp, RObj, Params>
where
    RObj: RTreeObject,
    Params: RTreeParams,
{
    fn from(plugin: SpatialPlugin<TComp, RTreeAccess<TComp, RObj, Params>>) -> Self {
        let tree: RTree<RObj, Params> = RTree::new_with_params();

        RTreeAccess {
            tree,
            min_moved: plugin.min_moved,
            recreate_after: plugin.recreate_after,
            component_type: PhantomData,
        }
    }
}
