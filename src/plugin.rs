use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    common::run_if_elapsed,
    resources_components::TimestepElapsed,
    spatial_access::{add_added, delete, update_moved},
    SpatialAccess,
};

pub struct SpatialPlugin<TComp, Access> {
    pub component_type: PhantomData<TComp>,
    pub spatial_access: PhantomData<Access>,
    pub min_moved: f32,
    pub recreate_after: usize,
    pub timestep: Option<f32>,
}

// apparently generic structs with PhantomData have issues with derive(Clone, Copy)
// see https://stackoverflow.com/a/60907370
impl<TComp, Access> Copy for SpatialPlugin<TComp, Access> {}

impl<TComp, Access> Clone for SpatialPlugin<TComp, Access> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TComp, Access> Default for SpatialPlugin<TComp, Access> {
    fn default() -> Self {
        SpatialPlugin {
            component_type: PhantomData,
            spatial_access: PhantomData,
            recreate_after: 100,
            min_moved: 1.0,
            timestep: Some(1.0 / 60.0), // update r-tree 60 times per second default
        }
    }
}

impl<TComp, Access> Plugin for SpatialPlugin<TComp, Access>
where
    TComp: Component + Send + Sync + 'static,
    Access: SpatialAccess + From<SpatialPlugin<TComp, Access>> + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        let tree_access = Access::from(*self);

        app.insert_resource(tree_access)
            .add_startup_system_to_stage(StartupStage::PostStartup, add_added::<Access>);

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
                    .with_system(add_added::<Access>)
                    .with_system(update_moved::<Access>)
                    .with_system(delete::<Access>),
            );
        } else {
            app.add_system_to_stage(CoreStage::PostUpdate, add_added::<Access>)
                .add_system_to_stage(CoreStage::PostUpdate, update_moved::<Access>)
                .add_system_to_stage(CoreStage::PostUpdate, delete::<Access>);
        }
    }
}
