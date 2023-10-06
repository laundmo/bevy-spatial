use bevy::prelude::*;

use crate::{TComp, point::SpatialPoint};

// todo: change Point to impl IntoPoint?
#[allow(clippy::module_name_repetitions)]
pub trait UpdateSpatialAccess: SpatialAccess {
    /// Updates the underlying datastructure
    ///
    /// The boolean indicates if the point needs to be updated or is a existing point.
    /// data should always include all points, even if they are not updated.
    /// This is for datastructures like ``KDTree``, which need to be fully rebuilt.
    fn update(
        &mut self,
        data: impl Iterator<Item = (Self::Point, bool)>,
        removed: impl Iterator<Item = Entity>,
    ) {
        for (p, changed) in data {
            if changed {
                self.remove_point(p);
                self.add(p);
            }
        }
        for e in removed {
            self.remove_entity(e);
        }
    }
    /// Adds the point to the underlying datastructure.
    fn add(&mut self, point: Self::Point);
    /// Remove the point by coordinate + entity from the underlying datastructure.
    fn remove_point(&mut self, point: Self::Point) -> bool;
    /// Remove the point by entity from the underlying datastructure.
    fn remove_entity(&mut self, entity: Entity) -> bool;
    /// Clear the underlying datastructure, removing all points it contains.
    fn clear(&mut self);
}

/// Trait for accessing point-based spatial datastructures.
pub trait SpatialAccess: Send + Sync + 'static {
    /// The point type, can be anything implementing [`SpatialPoint`].
    type Point: SpatialPoint;
    /// The marker component type marking the entities whos points are stored, used for accessing the component in trait bounds.
    type Comp: TComp;
    /// The type of a single query result.
    type ResultT;

    /// Get the nearest neighbour to `loc`.
    /// Be aware that that distance to the returned point will be zero if `loc` is part of the datastructure.
    fn nearest_neighbour(&self, loc: <Self::Point as SpatialPoint>::Vec) -> Option<Self::ResultT>;

    /// Return the k nearest neighbours to `loc`.
    fn k_nearest_neighbour(
        &self,
        loc: <Self::Point as SpatialPoint>::Vec,
        k: usize,
    ) -> Vec<Self::ResultT>;

    /// Return all points which are within the specified distance.
    fn within_distance(
        &self,
        loc: <Self::Point as SpatialPoint>::Vec,
        distance: <Self::Point as SpatialPoint>::Scalar,
    ) -> Vec<Self::ResultT>;

    /// Return all points which are within the specified rectangular axis-aligned region.
    fn within(
        &self,
        loc1: <Self::Point as SpatialPoint>::Vec,
        loc2: <Self::Point as SpatialPoint>::Vec,
    ) -> Vec<Self::ResultT>;
}

// TODO: SpatialAABBAccess trait definition - should it be separate from SpatialAccess or depend on it?
