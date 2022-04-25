use std::marker::PhantomData;

use rstar::{RTree, RTreeObject, RTreeParams};

use crate::common::EntityPoint;

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

impl<T, RObj, Params> Default for RTreeAccess<T, RObj, Params>
where
    RObj: RTreeObject,
    Params: RTreeParams,
{
    fn default() -> Self {
        let tree: RTree<RObj, Params> = RTree::new_with_params();
        RTreeAccess {
            tree,
            min_moved: 1.0,
            recreate_after: 100,
            component_type: PhantomData,
        }
    }
}

impl<Unit> PartialEq for EntityPoint<Unit>
where
    Unit: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}
