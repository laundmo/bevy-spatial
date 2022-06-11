use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    common::run_if_elapsed,
    resources_components::TimestepElapsed,
    spatial_access::{add_added, delete, update_moved},
    SpatialAccess,
};

/// The generic plugin struct which stores metadata for updating and recreating the choosen spatial index.
///
/// This bevy plugin is what keeps track of and updates the spatial index.
/// To use this directly, pass in a type that implements [`SpatialAccess`]
///
/// ```rust
/// #[derive(Component)]
/// struct NearestNeighbourComponent;
///
/// App::new().add_plugin(
///     SpatialPlugin::<NearestNeighbourComponent, KDTreeAccess<TComp, EntityPoint2D>> { ..default() },
/// )
/// ```
pub struct SpatialPlugin<TComp, Access> {
    #[doc(hidden)]
    pub component_type: PhantomData<TComp>,
    #[doc(hidden)]
    pub spatial_access: PhantomData<Access>,
    /// The minimum distance a entity has to move before its updated in the index. Increase this if small movements do not matter.
    pub min_moved: f32,
    /// The threshold of changes that have to happend within the same timestep or frame for the index to be completely recreated. After a certain point completely recreating can be more efficient.
    pub recreate_after: usize,
    /// Optional delay in seconds between update runs for the
    pub timestep: Option<f32>,
}

// apparently generic structs with PhantomData have issues with derive(Clone, Copy)
// hence they are implemented manually here.
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
            timestep: None,
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
            .add_startup_system_to_stage(StartupStage::PostStartup, add_added::<Access>)
            .add_system_to_stage(CoreStage::PostUpdate, delete::<Access>);

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
                    .with_system(update_moved::<Access>),
            );
        } else {
            app.add_system_to_stage(CoreStage::PostUpdate, add_added::<Access>)
                .add_system_to_stage(CoreStage::PostUpdate, update_moved::<Access>);
        }
    }
}
