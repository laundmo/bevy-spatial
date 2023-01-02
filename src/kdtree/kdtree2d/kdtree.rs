use bevy::{math::Vec3Swizzles, prelude::*};
use kd_tree::{KdPoint, KdTree};

use crate::{
    common::EntityPoint2D, kdtree::common::KDTreeAccess, spatial_access::TrackedQuery,
    SpatialAccess,
};

pub type KDTreeAccess2D<TComp> = KDTreeAccess<TComp, EntityPoint2D>;

// implement `KdPoint` for your item type.
impl KdPoint for EntityPoint2D {
    type Scalar = f32;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.vec[k]
    }
}

impl<TComp> SpatialAccess for KDTreeAccess<TComp, EntityPoint2D>
where
    TComp: Component + Sync + 'static,
{
    /// The component which this tree tracks.
    type TComp = TComp;

    /// Add entities to kd-tree, from bevy query. Used internally and called from a system.
    ///
    /// Due to kd-tree not supporting modification, this recreates the tree with all entities.
    fn update_tree(
        &mut self,
        mut _commands: Commands,
        query: Query<TrackedQuery<Self::TComp>, With<Self::TComp>>,
    ) {
        let _span = info_span!("add-added").entered();
        let all: Vec<(Vec3, Entity)> = query
            .iter()
            .map(|e| (e.transform.translation, e.entity))
            .collect();

        self.recreate(all);
    }

    /// No-op due to kd-tree not supporting modification.    
    fn delete(&mut self, _removed: RemovedComponents<Self::TComp>) {}

    /// Squared distance between 2 Vec3s.
    ///
    /// For 2d trees this will discard the z component of the Vec3.
    fn distance_squared(&self, loc_a: Vec3, loc_b: Vec3) -> f32 {
        loc_a.xy().distance_squared(loc_b.xy())
    }

    /// Get the nearest neighbour to a position.
    fn nearest_neighbour(&self, loc: Vec3) -> Option<(Vec3, Entity)> {
        let res = self.tree.nearest(&[loc.x, loc.y]);
        res.map(|point| (point.item.vec.extend(0.0), point.item.entity))
    }

    /// Get the `k` neighbours to `loc`
    ///
    /// If `loc` is the location of a tracked entity, you might want to skip the first.
    fn k_nearest_neighbour(&self, loc: Vec3, k: usize) -> Vec<(Vec3, Entity)> {
        let _span = info_span!("k-nearest").entered();

        return self
            .tree
            .nearests(&[loc.x, loc.y], k)
            .iter()
            .map(|e| (e.item.vec.extend(0.0), e.item.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }

    /// Get all entities within a certain distance (radius) of `loc`
    fn within_distance(&self, loc: Vec3, distance: f32) -> Vec<(Vec3, Entity)> {
        let _span = info_span!("within-distance").entered();

        if self.tree.len() == 0 {
            return vec![];
        }
        return self
            .tree
            .within_radius(&[loc.x, loc.y], distance)
            .iter()
            .map(|e| (e.vec.extend(0.0), e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }

    /// Recreates the tree with the provided entity locations/coordinates.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn recreate(&mut self, all: Vec<(Vec3, Entity)>) {
        let _span_d = info_span!("collect-data").entered();
        let data: Vec<EntityPoint2D> = { all.iter().map(|e| e.into()).collect() };
        _span_d.exit();
        let _span = info_span!("recreate").entered();
        #[cfg(not(target_arch = "wasm32"))]
        let tree: KdTree<EntityPoint2D> = KdTree::par_build_by_ordered_float(data);
        #[cfg(target_arch = "wasm32")]
        let tree: KdTree<EntityPoint2D> = KdTree::build_by_ordered_float(data);
        self.tree = tree;
    }

    /// Adds a point to the tree, used internally.
    ///
    /// No-op due to kd-tree not supporting modification.
    fn add_point(&mut self, _point: (Vec3, Entity)) {}

    /// Removes a point from the tree, used internally.
    ///
    /// No-op due to kd-tree not supporting modification.
    fn remove_point(&mut self, _point: (Vec3, Entity)) -> bool {
        false
    }

    /// Removed a point from the tree by its entity.
    ///
    /// No-op due to kd-tree not supporting modification.
    fn remove_entity(&mut self, _entity: Entity) -> bool {
        false
    }
    /// Size of the tree
    fn size(&self) -> usize {
        self.tree.len()
    }

    /// The distance after which a entity is updated in the tree
    ///
    /// Always zero due to kd-tree being recreated every frame.
    fn get_min_dist(&self) -> f32 {
        0.
    }

    /// Get the distance after which the tree is recreated.
    ///
    /// Always zero due to kd-tree being recreated every frame.
    fn get_recreate_after(&self) -> usize {
        0
    }
}
