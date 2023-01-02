use bevy::{math::Vec3Swizzles, prelude::*};
use rstar::{DefaultParams, PointDistance, RTree, RTreeObject, RTreeParams, AABB};

use crate::{common::EntityPoint2D, rtree::common::RTreeAccess, spatial_access::SpatialAccess};

pub type RTreeAccess2D<TComp, Params = DefaultParams> = RTreeAccess<TComp, EntityPoint2D, Params>;

impl<TComp, Params> SpatialAccess for RTreeAccess2D<TComp, Params>
where
    Params: RTreeParams,
    TComp: Component + Sync + 'static,
{
    /// The component which this tree tracks.
    type TComp = TComp;

    /// Squared distance between 2 Vec3s.
    ///
    /// For 2d trees this will discard the z component of the Vec3.
    fn distance_squared(&self, loc_a: Vec3, loc_b: Vec3) -> f32 {
        loc_a.xy().distance_squared(loc_b.xy())
    }

    /// Get the nearest neighbour to a position.
    fn nearest_neighbour(&self, loc: Vec3) -> Option<(Vec3, Entity)> {
        let res = self.tree.nearest_neighbor(&[loc.x, loc.y]);
        res.map(|point| (point.vec.extend(0.0), point.entity))
    }

    /// Get the `k` neighbours to `loc`
    ///
    /// If `loc` is the location of a tracked entity, you might want to skip the first.
    fn k_nearest_neighbour(&self, loc: Vec3, k: usize) -> Vec<(Vec3, Entity)> {
        let _span = info_span!("k-nearest").entered();

        return self
            .tree
            .nearest_neighbor_iter(&[loc.x, loc.y])
            .take(k)
            .map(|e| (e.vec.extend(0.0), e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }

    /// Get all entities within a certain distance (radius) of `loc`
    fn within_distance(&self, loc: Vec3, distance: f32) -> Vec<(Vec3, Entity)> {
        let _span = info_span!("within-distance").entered();

        return self
            .tree
            .locate_within_distance([loc.x, loc.y], distance.powi(2))
            .map(|e| (e.vec.extend(0.0), e.entity))
            .collect::<Vec<(Vec3, Entity)>>();
    }

    /// Recreates the tree with the provided entity locations/coordinates.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn recreate(&mut self, all: Vec<(Vec3, Entity)>) {
        let _span_d = info_span!("collect-data").entered();
        let data: Vec<EntityPoint2D> = all
            .iter()
            .map(|e| {
                self.last_pos_map.insert(e.1, e.0);
                e.into()
            })
            .collect();
        _span_d.exit();
        let _span = info_span!("recreate").entered();
        let tree: RTree<EntityPoint2D, Params> = RTree::bulk_load_with_params(data);
        self.tree = tree;
    }

    /// Adds a point to the tree.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn add_point(&mut self, point: (Vec3, Entity)) {
        self.last_pos_map.insert(point.1, point.0);
        self.tree.insert(point.into());
    }

    /// Removes a point from the tree.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn remove_point(&mut self, point: (Vec3, Entity)) -> bool {
        self.tree.remove(&point.into()).is_some()
    }

    /// Removed a point from the tree by its entity.
    ///
    /// Only use if manually updating, the plugin will overwrite changes.
    fn remove_entity(&mut self, entity: Entity) -> bool {
        // safe to unwrap because we only remove entities that have been previously added
        let last_pos = self.last_pos_map.remove(&entity).unwrap();
        self.tree.remove(&(last_pos, entity).into()).is_some()
    }

    /// Size of the tree
    fn size(&self) -> usize {
        self.tree.size()
    }

    /// Get the distance after which a entity is updated in the tree
    fn get_min_dist(&self) -> f32 {
        self.min_moved
    }

    /// Get the amount of entities which moved per frame after which the tree is fully recreated instead of updated.
    fn get_recreate_after(&self) -> usize {
        self.recreate_after
    }

    /// Get last tracked position of an entity
    fn get_last_pos(&self, entity: Entity) -> Option<&Vec3> {
        self.last_pos_map.get(&entity)
    }
}

impl RTreeObject for EntityPoint2D {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.vec.into())
    }
}

// TODO: currently somewhat duplicating the SpatialAccess distance calculation - how to resolve?
impl PointDistance for EntityPoint2D {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        self.vec.distance_squared(Vec2::from_slice(point))
    }
}
