use std::marker::PhantomData;

use bevy::prelude::{Entity, Resource, Vec3};
use bevy::utils::HashMap;
use rstar::{RTree, RTreeObject, RTreeParams};

use crate::plugin::SpatialPlugin;

#[derive(Resource)]
pub struct RTreeAccess<TComp, RObj, Params>
where
    RObj: RTreeObject,
    Params: RTreeParams,
{
    /// The underlying RTree struct.
    pub tree: RTree<RObj, Params>,
    /// The amount of entities which moved per frame after which the tree is fully recreated instead of updated.
    pub recreate_after: usize,
    /// The distance after which a entity is updated in the tree
    pub min_moved: f32,

    /// Internal map from entity to the corresponding last position
    /// (used to make sure that removal logic is correct, and to check for significant moves)
    pub last_pos_map: HashMap<Entity, Vec3>,

    #[doc(hidden)]
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
            last_pos_map: Default::default(),
            component_type: PhantomData,
        }
    }
}
