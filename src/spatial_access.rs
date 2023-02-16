use bevy::prelude::*;

use crate::{
    datacontainer::{SpatialData, TComp},
    point::SpatialPoint,
};

trait UpdateSpatialAccess: SpatialAccess {
    /// Rebuilds the underlying datastructure fully
    fn rebuild(&mut self, data: &SpatialData<Self::Point, Self::Comp>) {
        self.clear();
        for (e, p) in data.all.iter() {
            self.add(*p, *e);
        }
    }
    /// Adds the point to the underlying datastructure.
    fn add(&mut self, point: Self::Point, entity: Entity);
    /// Remove the point by coordinate + entity from the underlying datastructure.
    fn remove_point(&mut self, point: Self::Point, entity: Entity) -> bool;
    /// Remove the point by entity from the underlying datastructure.
    fn remove_entity(&mut self, entity: Entity) -> bool;
    /// Clear the underlying datastructure, removing all points it contains.
    fn clear(&mut self);
}

pub trait SpatialAccess {
    type Point: SpatialPoint;
    type Comp: TComp;
    type ResultT;

    /// Get the nearest neighbour to `loc`.
    /// Be aware that that distance to the returned point will be zero if `loc` is part of the datastructure.
    fn nearest_neighbour(
        &self,
        loc: <Self::Point as SpatialPoint>::Vec,
    ) -> Option<(<Self::Point as SpatialPoint>::Vec, Option<Entity>)>;
    /// Return the k nearest neighbours to `loc`.
    fn k_nearest_neighbour(
        &self,
        loc: <Self::Point as SpatialPoint>::Vec,
        k: usize,
    ) -> Self::ResultT;
    /// Return all points which are within the specified distance.
    fn within_distance(
        &self,
        loc: <Self::Point as SpatialPoint>::Vec,
        distance: <Self::Point as SpatialPoint>::Scalar,
    ) -> Self::ResultT;
    /// Recreate the underlying datastructure with `all` points.
    fn update(&mut self, all: &SpatialData<Self::Point, Self::Comp>);
}

// TODO: SpatialAABBAccess trait definition - should it be separate from SpatialAccess or depend on it?
