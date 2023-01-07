use bevy::prelude::*;

use crate::point::SpatialPoint;

pub trait Metadata {
    /// Get the minimum distance that a entity has to travel before being updated in the datastructure.
    fn get_min_dist(&self) -> f32;
    /// Get the amount of moved/changed/added entities after which to perform a full recreate.
    fn get_recreate_after(&self) -> usize;
    /// Get the size of the underlying datastructure. Should match the number of tracked elements.
    fn size(&self) -> usize;
}

pub trait SpatialAccess {
    type Point: SpatialPoint;

    /// Get the nearest neighbour to `loc`.
    /// Be aware that that distance to the returned point will be zero if `loc` is part of the datastructure.
    fn nearest_neighbour(&self, loc: Self::Point) -> Option<(Self::Point, Entity)>;
    /// Return the k nearest neighbours to `loc`.
    fn k_nearest_neighbour(&self, loc: Self::Point, k: usize) -> Vec<(Self::Point, Entity)>;
    /// Return all points which are within the specified distance.
    fn within_distance(
        &self,
        loc: Self::Point,
        distance: <<Self as SpatialAccess>::Point as SpatialPoint>::Unit,
    ) -> Vec<(Self::Point, Entity)>;
    /// Recreate the underlying datastructure with `all` points.
    fn recreate(&mut self, all: Vec<(Self::Point, Entity)>);
    /// Adds the point to the underlying datastructure.
    fn add_point(&mut self, point: (Self::Point, Entity));
    /// Remove the point by coordinate + entity from the underlying datastructure.
    fn remove_point(&mut self, point: (Self::Point, Entity)) -> bool;
    /// Remove the point by entity from the underlying datastructure.
    fn remove_entity(&mut self, entity: Entity) -> bool;
}
