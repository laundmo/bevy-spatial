use bevy::{
    math::{DVec2, Vec3Swizzles},
    prelude::*,
};
use kd_tree::{KdPoint, KdTree as BaseKdTree};
use typenum::U2;

use crate::{
    datacontainer::SpatialData,
    kdtree::common::KDTree,
    point::{Point2, PointD2, SpatialPoint},
    spatial_access::SpatialAccess,
};

pub type KDTreeAccess2D<TComp> = KDTree<TComp, Point2>;

pub type KDTreeAccessD2D<TComp> = KDTree<TComp, PointD2>;

impl<TComp, Point> SpatialAccess for KDTree<TComp, Point>
where
    TComp: Component + Sync + 'static,
    Point: SpatialPoint + KdPoint + From<<Point as SpatialPoint>::Vec>,
    <Point as KdPoint>::Scalar: From<<Point as SpatialPoint>::Scalar>,
{
    type Point = Point;

    type Comp = TComp;
    type ResultT = Vec<(<Point as SpatialPoint>::Vec, Option<Entity>)>;

    /// Get the nearest neighbour to a position.
    fn nearest_neighbour(
        &self,
        loc: <Point as SpatialPoint>::Vec,
    ) -> Option<(<Point as SpatialPoint>::Vec, Option<Entity>)> {
        let p: Point = loc.into();
        let res = self.tree.nearest(&p);
        res.map(|point| (point.item.vec(), point.item.entity()))
    }

    /// Get the `k` neighbours to `loc`
    ///
    /// If `loc` is the location of a tracked entity, you might want to skip the first.
    fn k_nearest_neighbour(&self, loc: <Point as SpatialPoint>::Vec, k: usize) -> Self::ResultT {
        let _span = info_span!("k-nearest").entered();
        let p: Point = loc.into();

        self.tree
            .nearests(&p, k)
            .iter()
            .map(|e| (e.item.vec(), e.item.entity()))
            .collect()
    }

    /// Get all entities within a certain distance (radius) of `loc`
    fn within_distance(
        &self,
        loc: <Point as SpatialPoint>::Vec,
        distance: <Point as SpatialPoint>::Scalar,
    ) -> Self::ResultT {
        let _span = info_span!("within-distance").entered();

        let distance: <Point as KdPoint>::Scalar = distance.into();

        if self.tree.len() == 0 {
            vec![]
        } else {
            let p: Point = loc.into();

            self.tree
                .within_radius(&p, distance)
                .iter()
                .map(|e| (e.vec(), e.entity()))
                .collect()
        }
    }

    fn update(&mut self, data: &SpatialData<Self::Point, Self::Comp>) {
        todo!()
    }
}

// /// The component which this tree tracks.
// type TComp = TComp;

// /// No-op due to kd-tree not supporting modification.
// fn update_moved(
//     &mut self,
//     mut _set: ParamSet<(
//         Query<TrackedQuery<Self::TComp>, Changed<Transform>>,
//         Query<TrackedQuery<Self::TComp>>,
//     )>,
// ) {
// }

// /// Add entities to kd-tree, from bevy query. Used internally and called from a system.
// ///
// /// Due to kd-tree not supporting modification, this recreates the tree with all entities.
// fn add_added(
//     &mut self,
//     mut _commands: Commands,
//     all_query: Query<(Entity, &Transform), With<Self::TComp>>,
//     _added_query: Query<(Entity, &Transform), Added<Self::TComp>>,
// ) {
//     let _span = info_span!("add-added").entered();
//     let all: Vec<(Vec3, Entity)> = all_query.iter().map(|i| (i.1.translation, i.0)).collect();

//     self.recreate(all);
// }

// /// No-op due to kd-tree not supporting modification.
// fn delete(&mut self, _removed: RemovedComponents<Self::TComp>) {}

// /// Squared distance between 2 Vec3s.
// ///
// /// For 2d trees this will discard the z component of the Vec3.
// fn distance_squared(&self, loc_a: Vec3, loc_b: Vec3) -> f32 {
//     loc_a.xy().distance_squared(loc_b.xy())
// }
// /// Recreates the tree with the provided entity locations/coordinates.
// ///
// /// Only use if manually updating, the plugin will overwrite changes.
// fn recreate(&mut self, all: Vec<(Vec3, Entity)>) {
//     let _span_d = info_span!("collect-data").entered();
//     let data: Vec<EntityPoint2D> = { all.iter().map(|e| e.into()).collect() };
//     _span_d.exit();
//     let _span = info_span!("recreate").entered();
//     #[cfg(not(target_arch = "wasm32"))]
//     let tree: KdTree<EntityPoint2D> = KdTree::par_build_by_ordered_float(data);
//     #[cfg(target_arch = "wasm32")]
//     let tree: KdTree<EntityPoint2D> = KdTree::build_by_ordered_float(data);
//     self.tree = tree;
// }

// /// Adds a point to the tree, used internally.
// ///
// /// No-op due to kd-tree not supporting modification.
// fn add_point(&mut self, _point: (Vec3, Entity)) {}

// /// Removes a point from the tree, used internally.
// ///
// /// No-op due to kd-tree not supporting modification.
// fn remove_point(&mut self, _point: (Vec3, Entity)) -> bool {
//     false
// }

// /// Removed a point from the tree by its entity.
// ///
// /// No-op due to kd-tree not supporting modification.
// fn remove_entity(&mut self, _entity: Entity) -> bool {
//     false
// }
// /// Size of the tree
// fn size(&self) -> usize {
//     self.tree.len()
// }

// /// The distance after which a entity is updated in the tree
// ///
// /// Always zero due to kd-tree being recreated every frame.
// fn get_min_dist(&self) -> f32 {
//     0.
// }

// /// Get the distance after which the tree is recreated.
// ///
// /// Always zero due to kd-tree being recreated every frame.
// fn get_recreate_after(&self) -> usize {
//     0
// }
