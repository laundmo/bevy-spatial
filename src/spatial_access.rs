use bevy::{
    ecs::{query::WorldQuery, system::Resource},
    prelude::*,
};

use crate::resources_components::MovementTracked;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TrackedQuery<'a, TComp>
where
    TComp: Sync + Send + 'static,
{
    pub entity: Entity,
    pub transform: &'a Transform,
    pub tracker: &'static mut MovementTracked<TComp>,
}

pub trait SpatialAccess
where
    <Self as SpatialAccess>::TComp: Component + Sync + 'static,
{
    type TComp;

    /// Update moved entities in datastructure, from bevy query. Used internally and called from a system.
    fn update_moved(
        &mut self,
        mut set: ParamSet<(
            Query<TrackedQuery<Self::TComp>, Changed<Transform>>,
            Query<TrackedQuery<Self::TComp>>,
        )>,
    ) {
        let move_dist = info_span!(
            "compute_moved_significant_distance",
            name = "compute_moved_significant_distance"
        )
        .entered();
        let moved: Vec<(Entity, Vec3, Vec3)> = set
            .p0()
            .iter()
            .filter(|e| {
                self.distance_squared(e.transform.translation, e.tracker.lastpos)
                    >= self.get_min_dist().powi(2)
            })
            .map(|e| (e.entity, e.tracker.lastpos, e.transform.translation))
            .collect();
        move_dist.exit();

        if moved.len() >= self.get_recreate_after() {
            let recreate = info_span!("recreate_with_all", name = "recreate_with_all").entered();
            let all: Vec<(Vec3, Entity)> = set
                .p1()
                .iter_mut()
                .map(|mut tqi| {
                    let r = (tqi.transform.translation, tqi.entity);
                    tqi.tracker.lastpos = r.0;
                    r
                })
                .collect();

            self.recreate(all);
            recreate.exit();
        } else {
            let update = info_span!("partial_update", name = "partial_update").entered();
            let mut p1 = set.p1();
            for (entity, lastpos, curpos) in moved {
                let mut mut_tracked = p1.get_mut(entity).unwrap();
                if self.remove_point((lastpos, entity)) {
                    self.add_point((curpos, entity));
                    mut_tracked.tracker.lastpos = curpos;
                }
            }
            update.exit();
        }
    }

    /// Add entities which recently had the tracked component added. Used internally and called from a system.
    fn add_added(
        &mut self,
        mut commands: Commands,
        all_query: Query<(Entity, &Transform), With<Self::TComp>>,
        added_query: Query<(Entity, &Transform), Added<Self::TComp>>,
    ) {
        let added: Vec<(Entity, &Transform)> = added_query.iter().collect();

        if added.len() >= self.size() / 2 {
            let recreate = info_span!("recreate_with_all", name = "recreate_with_all").entered();

            let all: Vec<(Vec3, Entity)> = all_query
                .iter()
                .map(|(entity, pos)| {
                    let r = (pos.translation, entity);
                    commands
                        .entity(r.1)
                        .insert(MovementTracked::<Self::TComp>::new(r.0));
                    r
                })
                .collect();

            self.recreate(all);
            recreate.exit();
        } else {
            let update = info_span!("partial_update", name = "partial_update").entered();

            for (entity, transform) in added {
                self.add_point((transform.translation, entity));
                commands
                    .entity(entity)
                    .insert(MovementTracked::<Self::TComp>::new(transform.translation));
            }
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

pub fn update_moved<SAcc>(
    mut acc: ResMut<SAcc>,
    set: ParamSet<(
        Query<TrackedQuery<SAcc::TComp>, Changed<Transform>>,
        Query<TrackedQuery<SAcc::TComp>>,
    )>,
) where
    SAcc: SpatialAccess + Resource + Sync,
{
    acc.update_moved(set);
}

pub fn add_added<SAcc>(
    mut acc: ResMut<SAcc>,
    commands: Commands,
    all_query: Query<(Entity, &Transform), With<SAcc::TComp>>,
    added_query: Query<(Entity, &Transform), Added<SAcc::TComp>>,
) where
    SAcc: SpatialAccess + Resource + Sync,
{
    acc.add_added(commands, all_query, added_query);
}

pub fn delete<SAcc>(mut acc: ResMut<SAcc>, removed: RemovedComponents<SAcc::TComp>)
where
    SAcc: SpatialAccess + Resource + Sync,
{
    acc.delete(removed);
}
