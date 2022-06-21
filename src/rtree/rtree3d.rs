use bevy::prelude::*;
use rstar::{PointDistance, RTree, RTreeObject, RTreeParams, AABB};

use crate::{common::EntityPoint3D, rtree::common::RTreeAccess, spatial_access::SpatialAccess};

pub type RTreeAccess3D<TComp, Params> = RTreeAccess<TComp, EntityPoint3D, Params>;

impl<TComp, Params> SpatialAccess for RTreeAccess3D<TComp, Params>
where
    Params: RTreeParams,
    TComp: Component + Sync + 'static,
{
    type TComp = TComp;
    fn distance_squared(&self, loc_a: Vec3, loc_b: Vec3) -> f32 {
        loc_a.distance_squared(loc_b)
    }

    // needs impl due to underlying datastructure access
    fn nearest_neighbour(&self, loc: Vec3) -> Option<(Vec3, Entity)> {
        let res = self.tree.nearest_neighbor(&[loc.x, loc.y, loc.z]);
        res.map(|point| (point.pos, point.entity))
    }
    fn k_nearest_neighbour(&self, loc: Vec3, k: usize) -> Vec<(Vec3, Entity)> {
        return self
            .tree
            .nearest_neighbor_iter(&[loc.x, loc.y, loc.z])
            .take(k)
            .map(|e| (e.pos, e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }
    fn within_distance(&self, loc: Vec3, distance: f32) -> Vec<(Vec3, Entity)> {
        return self
            .tree
            .locate_within_distance([loc.x, loc.y, loc.z], distance.powi(2))
            .map(|e| (e.pos, e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }
    fn recreate(&mut self, all: Vec<(Vec3, Entity)>) {
        let tree: RTree<EntityPoint3D, Params> =
            RTree::bulk_load_with_params(all.iter().map(|e| e.into()).collect());
        self.tree = tree;
    }
    fn add_point(&mut self, point: (Vec3, Entity)) {
        self.tree.insert(point.into())
    }
    fn remove_point(&mut self, point: (Vec3, Entity)) -> bool {
        self.tree.remove(&point.into()).is_some()
    }
    fn remove_entity(&mut self, entity: Entity) -> bool {
        self.tree.remove(&entity.into()).is_some()
    }
    fn size(&self) -> usize {
        self.tree.size()
    }
    fn get_min_dist(&self) -> f32 {
        self.min_moved
    }
    fn get_recreate_after(&self) -> usize {
        self.recreate_after
    }
}

impl RTreeObject for EntityPoint3D {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.pos.into())
    }
}

impl PointDistance for EntityPoint3D {
    fn distance_2(&self, point: &[f32; 3]) -> f32 {
        self.pos.distance_squared(Vec3::from_slice(point))
    }
}
