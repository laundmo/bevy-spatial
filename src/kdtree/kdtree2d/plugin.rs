use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    common::{run_if_elapsed, EntityPoint2D},
    resources_components::TimestepElapsed,
    spatial_access::{add_added, delete, update_moved},
};

use super::kdtree::KDTreeAccess;

pub struct KDTreePlugin2D<TComp> {
    pub component_type: PhantomData<TComp>,
    pub min_moved: f32,
    pub timestep: Option<f32>,
}

impl<TComp> Default for KDTreePlugin2D<TComp> {
    fn default() -> Self {
        KDTreePlugin2D {
            component_type: PhantomData,
            min_moved: 1.0,
            timestep: Some(1.0 / 60.0), // update r-tree 60 times per second default
        }
    }
}

impl<TComp> Plugin for KDTreePlugin2D<TComp>
where
    TComp: Component + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        let tree_access = KDTreeAccess::<TComp, EntityPoint2D> {
            min_moved: self.min_moved,
            ..default()
        };

        app.insert_resource(tree_access)
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                add_added::<KDTreeAccess<TComp, EntityPoint2D>>,
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
                    .with_system(add_added::<KDTreeAccess<TComp, EntityPoint2D>>)
                    .with_system(update_moved::<KDTreeAccess<TComp, EntityPoint2D>>)
                    .with_system(delete::<KDTreeAccess<TComp, EntityPoint2D>>),
            );
        } else {
            app.add_system_to_stage(
                CoreStage::PostUpdate,
                add_added::<KDTreeAccess<TComp, EntityPoint2D>>,
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_moved::<KDTreeAccess<TComp, EntityPoint2D>>,
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                delete::<KDTreeAccess<TComp, EntityPoint2D>>,
            );
        }
    }
}
