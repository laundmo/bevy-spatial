use bevy::{
    ecs::{query::WorldQuery, system::Resource},
    prelude::*,
};

use crate::resources_components::MovementTracked;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TrackedQuery<'a, TComp>
where
    TComp: Component + Sync + Send + 'static,
{
    pub entity: Entity,
    pub transform: &'a Transform,
    pub change_tracker: ChangeTrackers<Transform>,
    pub added_tracker: ChangeTrackers<TComp>,
    pub movement_tracker: Option<&'static mut MovementTracked<TComp>>,
}

pub trait SpatialAccess
where
    <Self as SpatialAccess>::TComp: Component + Sync + 'static,
{
    type TComp;

    /// Update the data structure by adding:
    /// - new entities
    /// - entities that moved more than `self.get_min_dist()`
    ///
    /// If the number of new/updated entities is larger than `self.get_recreate_after()`,
    /// the whole data structure gets re-created from scratch.
    ///
    /// Used internally and called from a system.
    fn update_tree(
        &mut self,
        mut commands: Commands,
        mut query: Query<TrackedQuery<Self::TComp>, With<Self::TComp>>,
    ) {

        // get added entities, and add a MovementTracker
        let added_dist = info_span!(
            "compute_added_entities",
            name = "compute_added_entities"
        ).entered();
        let added: Vec<_> = query.iter()
            .filter(|e| e.added_tracker.is_added())
            .map(|e| {
                commands.entity(e.entity)
                    .insert(MovementTracked::<Self::TComp>::new(e.transform.translation));
                (e.transform.translation, e.entity)
            })
            .collect();
        added_dist.exit();


        // get already existing entities that moved a significant distance, and update their
        // last position in the `movement_tracker`
        let move_dist = info_span!(
            "compute_moved_significant_distance",
            name = "compute_moved_significant_distance"
        )
        .entered();
        let moved: Vec<_> = query
            .iter_mut()
            .filter(|e| {
                if e.change_tracker.is_changed() && !e.added_tracker.is_added() {
                    // optimization if distance deltas do not matter
                    if self.get_min_dist() <= 0.0 {
                        return true
                    }
                    // movement_tracker will always be present at this point
                    return self.distance_squared(
                        e.transform.translation,
                        e.movement_tracker.as_ref().unwrap().lastpos) >= self.get_min_dist().powi(2)
                }
                false
            })
            .map(|e| {
                // update the movement tracker
                e.movement_tracker.unwrap().lastpos = e.transform.translation;
                (e.transform.translation, e.entity)
            })
            .collect();
        move_dist.exit();

        if added.len() + moved.len() >= self.get_recreate_after() {
            let recreate = info_span!("recreate_with_all", name = "recreate_with_all").entered();
            let all: Vec<(Vec3, Entity)> = query
                .iter()
                .map(|e| (e.transform.translation, e.entity))
                .collect();

            self.recreate(all);
            recreate.exit();
        } else {
            let update = info_span!("partial_update", name = "partial_update").entered();
            added.into_iter().for_each(|(curpos, entity)| self.add_point((curpos, entity)));
            moved.into_iter().for_each(|(curpos, entity)| {
                if self.remove_entity(entity) {
                    self.add_point((curpos, entity));
                }
            });
            update.exit();
        }
    }


    /// Delete despawned entities from datastructure. Used internally and called from a system.
    fn delete(&mut self, removed: RemovedComponents<Self::TComp>) {
        for entity in removed.iter() {
            self.remove_entity(entity);
        }
    }

    /// Get the squared distance using the calculation that implementation uses.
    /// Mainly a trait method due to 2d structures using Vec3 but ignoring the 3rd dimension for distance calculations.
    fn distance_squared(&self, loc_a: Vec3, loc_b: Vec3) -> f32;

    /// Get the nearest neighbour to `loc`.
    /// Be aware that that distance to the returned point will be zero if `loc` is part of the datastructure.
    fn nearest_neighbour(&self, loc: Vec3) -> Option<(Vec3, Entity)>;

    /// Return the k nearest neighbours to `loc`.
    fn k_nearest_neighbour(&self, loc: Vec3, k: usize) -> Vec<(Vec3, Entity)>;
    /// Return all points which are within the specified distance.
    fn within_distance(&self, loc: Vec3, distance: f32) -> Vec<(Vec3, Entity)>;
    /// Recreate the underlying datastructure with `all` points.
    fn recreate(&mut self, all: Vec<(Vec3, Entity)>);
    /// Adds the point to the underlying datastructure.
    fn add_point(&mut self, point: (Vec3, Entity));
    /// Remove the point by coordinate + entity from the underlying datastructure.
    fn remove_point(&mut self, point: (Vec3, Entity)) -> bool;
    /// Remove the point by entity from the underlying datastructure.
    fn remove_entity(&mut self, entity: Entity) -> bool;
    /// Get the size of the underlying datastructure. Should match the number of tracked elements.
    fn size(&self) -> usize;
    /// Get the minimum distance that a entity has to travel before being updated in the datastructure.
    fn get_min_dist(&self) -> f32;
    /// Get the amount of moved/changed/added entities after which to perform a full recreate.
    fn get_recreate_after(&self) -> usize;
}

pub fn update_tree<SAcc>(
    mut acc: ResMut<SAcc>,
    commands: Commands,
    query: Query<TrackedQuery<SAcc::TComp>, With<SAcc::TComp>>,
) where
    SAcc: SpatialAccess + Resource + Sync,
{
    acc.update_tree(commands, query);
}


pub fn delete<SAcc>(mut acc: ResMut<SAcc>, removed: RemovedComponents<SAcc::TComp>)
where
    SAcc: SpatialAccess + Resource + Sync,
{
    acc.delete(removed);
}
