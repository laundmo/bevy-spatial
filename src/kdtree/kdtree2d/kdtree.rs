use bevy::{math::Vec3Swizzles, prelude::*};
use kd_tree::{KdPoint, KdTree};

use crate::{
    common::EntityPoint2D, kdtree::common::KDTreeAccess, spatial_access::TrackedQuery,
    SpatialAccess,
};

pub type KDTreeAccess2D<TComp> = KDTreeAccess<TComp, EntityPoint2D>;

impl KdPoint for EntityPoint2D {
    type Scalar = f32;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.pos[k]
    }
}

impl<TComp> SpatialAccess for KDTreeAccess<TComp, EntityPoint2D>
where
    TComp: Component + Sync + 'static,
{
    type TComp = TComp;

    /// noop due to kd-tree not supporting edits.
    fn update_moved(
        &mut self,
        mut _set: ParamSet<(
            Query<TrackedQuery<Self::TComp>, Changed<Transform>>,
            Query<TrackedQuery<Self::TComp>>,
        )>,
    ) {
    }

    fn add_added(
        &mut self,
        mut _commands: Commands,
        all_query: Query<(Entity, &Transform), With<Self::TComp>>,
        _added_query: Query<(Entity, &Transform), Added<Self::TComp>>,
    ) {
        let all: Vec<(Vec3, Entity)> = all_query.iter().map(|i| (i.1.translation, i.0)).collect();

        self.recreate(all);
    }

    /// noop due to kd-tree not supporting edits.
    fn delete(&mut self, _removed: RemovedComponents<Self::TComp>) {}

    // needs impl due to 2d ignoring the z part of vec3
    fn distance_squared(&self, loc_a: Vec3, loc_b: Vec3) -> f32 {
        loc_a.xy().distance_squared(loc_b.xy())
    }

    // needs impl due to underlying datastructure access
    fn nearest_neighbour(&self, loc: Vec3) -> Option<(Vec3, Entity)> {
        let res = self.tree.nearest(&[loc.x, loc.y]);
        res.map(|point| (point.item.pos.extend(0.0), point.item.entity))
    }
    fn k_nearest_neighbour(&self, loc: Vec3, k: usize) -> Vec<(Vec3, Entity)> {
        return self
            .tree
            .nearests(&[loc.x, loc.y], k)
            .iter()
            .map(|e| (e.item.pos.extend(0.0), e.item.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }
    fn within_distance(&self, loc: Vec3, distance: f32) -> Vec<(Vec3, Entity)> {
        if self.tree.len() == 0 {
            return vec![];
        }
        return self
            .tree
            .within_radius(&[loc.x, loc.y], distance)
            .iter()
            .map(|e| (e.pos.extend(0.0), e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }

    fn recreate(&mut self, all: Vec<(Vec3, Entity)>) {
        // info!("recreate {:?}", all.len());
        let tree: KdTree<EntityPoint2D> =
            KdTree::build_by_ordered_float(all.iter().map(|e| e.into()).collect());
        self.tree = tree;
    }

    /// noop due to kd-tree not supporting edits.
    fn add_point(&mut self, _point: (Vec3, Entity)) {}

    /// noop due to kd-tree not supporting edits.
    fn remove_point(&mut self, _point: (Vec3, Entity)) -> bool {
        false
    }

    /// awlways false due to kd-tree not supporting edits.
    fn remove_entity(&mut self, _entity: Entity) -> bool {
        false
    }
    fn size(&self) -> usize {
        self.tree.len()
    }
    fn get_min_dist(&self) -> f32 {
        self.min_moved
    }

    /// awlways 0 due to kd-tree not supporting edits.
    fn get_recreate_after(&self) -> usize {
        0
    }
}
