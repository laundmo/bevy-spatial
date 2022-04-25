use bevy::prelude::*;
use rstar::{RTree, RTreeParams};

use crate::rtree::{components::MovementTracked, resources::MinMoved};

use super::rtree_obj::{EntityPoint3D, TreeAccess3D};

pub type RTreeTracked3D<T> = MovementTracked<T, Vec3>;

pub fn add_rtree<TComp: Component + Send + Sync, Params: RTreeParams + 'static>(
    mut taccess: ResMut<TreeAccess3D<TComp, Params>>,
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<TComp>>,
    query_all: Query<(Entity, &Transform), With<TComp>>,
) {
    // get vec of all added EntityPoint3d, for which the entities have already been given the tracked component
    let added: Vec<EntityPoint3D> = query
        .iter()
        .map(|(entity, transform)| {
            let pos = transform.translation;
            commands
                .entity(entity)
                .insert(RTreeTracked3D::<TComp>::new(pos));
            (entity, pos).into()
        })
        .collect();

    if added.len() >= taccess.tree.size() {
        // if theres more than the current tree size added, recreate tree from ALL.
        let tree: RTree<_, Params> =
            RTree::bulk_load_with_params(query_all.iter().map(|e| e.into()).collect());
        taccess.tree = tree;
    } else {
        for point in added {
            taccess.tree.insert(point);
            info!("Added {:?}", point.entity);
        }
    }
}

pub fn moved_rtree<TComp: Component + Send + Sync, Params: RTreeParams + 'static>(
    min_moved: Res<MinMoved<TComp>>,
    mut taccess: ResMut<TreeAccess3D<TComp, Params>>,
    moved_query: Query<(Entity, &Transform), (With<RTreeTracked3D<TComp>>, Changed<Transform>)>,
    mut tracked: Query<&mut RTreeTracked3D<TComp>>,
    query_all: Query<(Entity, &Transform), With<TComp>>,
) {
    let moved: Vec<EntityPoint3D> = moved_query
        .iter()
        .filter_map(|(entity, transform)| {
            let rtreetracked = tracked.get_mut(entity).unwrap();
            if transform.translation.distance_squared(rtreetracked.lastpos)
                >= min_moved.dist.powi(2)
            {
                return Some((entity, transform).into());
            } else {
                return None;
            }
        })
        .collect();

    if moved.len() >= taccess.tree.size() {
        let tree: RTree<_, Params> =
            RTree::bulk_load_with_params(query_all.iter().map(|e| e.into()).collect());
        for point in moved {
            let mut rtreetracked = tracked.get_mut(point.entity).unwrap();
            rtreetracked.lastpos = point.vec;
        }
        taccess.tree = tree;
        info!("Recreated Tree!");
    } else {
        for point in moved {
            let mut rtreetracked = tracked.get_mut(point.entity).unwrap();
            if taccess
                .tree
                .remove(&(rtreetracked.lastpos, point.entity).into())
                .is_none()
            {
                info!("issue removing {:?} \n {:?}", point.entity, taccess.tree);
            } else {
                taccess.tree.insert(point);
                rtreetracked.lastpos = point.vec;
                info!("updated {:?}", point.entity);
            }
        }
    }
}

pub fn delete_rtree<TComp: Component + Send + Sync, Params: RTreeParams + 'static>(
    mut taccess: ResMut<TreeAccess3D<TComp, Params>>,
    removed: RemovedComponents<TComp>,
) {
    for entity in removed.iter() {
        info!("attempting removal {:?}", entity);
        if taccess.tree.remove(&entity.into()).is_some() {
            info!("successfully deleted {:?}", entity);
        }
    }
}
