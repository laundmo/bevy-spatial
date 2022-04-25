use std::marker::PhantomData;

use bevy::prelude::*;
use rstar::RTreeParams;

use crate::{
    common::run_if_elapsed,
    resources_components::TimestepElapsed,
    spatial_access::{add_added, delete, update_moved},
};

use super::rtree_obj::TreeAccess2D;

pub struct RTreePlugin2D<TComp, Params> {
    pub component_type: PhantomData<TComp>,
    pub params: PhantomData<Params>,
    pub min_moved: f32,
    pub recreate_after: usize,
    pub timestep: Option<f32>,
}

impl<TComp, Params> Default for RTreePlugin2D<TComp, Params> {
    fn default() -> Self {
        RTreePlugin2D {
            component_type: PhantomData,
            params: PhantomData,
            min_moved: 1.0,
            recreate_after: 100,
            timestep: Some(1.0 / 60.0), // update r-tree 60 times per second default
        }
    }
}

impl<TComp, Params> Plugin for RTreePlugin2D<TComp, Params>
where
    TComp: Component + Send + Sync + 'static,
    Params: RTreeParams + 'static,
{
    fn build(&self, app: &mut App) {
        let tree_access = TreeAccess2D::<TComp, Params> {
            min_moved: self.min_moved,
            recreate_after: self.recreate_after,
            ..default()
        };

        app.insert_resource(tree_access)
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                add_added::<TreeAccess2D<TComp, Params>>,
            );

        // decide whether to use the timestep
        if let Some(step) = self.timestep {
            app.insert_resource(TimestepElapsed::<TComp>(
                Timer::from_seconds(step, false),
                PhantomData,
            ));
            app.add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_elapsed::<TComp>)
                    .with_system(add_added::<TreeAccess2D<TComp, Params>>)
                    .with_system(update_moved::<TreeAccess2D<TComp, Params>>)
                    .with_system(delete::<TreeAccess2D<TComp, Params>>),
            );
        } else {
            app.add_system_to_stage(
                CoreStage::PostUpdate,
                add_added::<TreeAccess2D<TComp, Params>>,
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_moved::<TreeAccess2D<TComp, Params>>,
            )
            .add_system_to_stage(CoreStage::PostUpdate, delete::<TreeAccess2D<TComp, Params>>);
        }
    }
}
