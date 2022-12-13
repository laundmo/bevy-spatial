use std::marker::PhantomData;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    common::run_if_elapsed,
    resources_components::TimestepElapsed,
    spatial_access::{add_added, delete, update_moved},
    SpatialAccess,
};

/// The core plugin struct which stores metadata for updating and recreating the choosen spatial index.
///
/// Generics:
///   - `TComp` should be the marker Component which are tracked by this plugin.
///   - `Access` is the type implementing SpatialAccess.
///
///
/// You should use the following type aliases from their respective features instead.
///
/// | feature | Plugin |
/// | ------- | ------ |
/// | kdtree  | [KDTreePlugin2D](crate::KDTreePlugin2D) |
/// | rstar   | [RTreePlugin2D](crate::RTreePlugin2D) or [RTreePlugin3D](crate::RTreePlugin3D) |
pub struct SpatialPlugin<TComp, Access> {
    pub component_type: PhantomData<TComp>,
    pub spatial_access: PhantomData<Access>,
    /// The minimum distance a entity has to move before its updated in the index. Increase this if small movements do not matter.
    pub min_moved: f32,
    /// The threshold of changes that have to happend within the same timestep or frame for the index to be completely recreated. After a certain point completely recreating can be more efficient.
    pub recreate_after: usize,
    /// Optional delay in seconds between tree updates. Increasing this means distances might be older than one frame.
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
            timestep: None,
        }
    }
}

impl<TComp, Access> Plugin for SpatialPlugin<TComp, Access>
where
    TComp: Component + Send + Sync + 'static,
    Access: Resource + SpatialAccess + From<SpatialPlugin<TComp, Access>> + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        let tree_access = Access::from(*self);

        app.insert_resource(tree_access)
            .add_startup_system_to_stage(StartupStage::PostStartup, add_added::<Access>)
            .add_system_to_stage(CoreStage::PostUpdate, delete::<Access>);

        // decide whether to use the timestep
        if let Some(step) = self.timestep {
            app.insert_resource(TimestepElapsed::<TComp>(
                Timer::from_seconds(step, TimerMode::Once),
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

pub struct SpatialPlugins<TComp>(PhantomData<TComp>);

impl<TComp> PluginGroup for SpatialPlugins<TComp> {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(CoordinateExtractPlugin::<TComp>)
            .add(KDTreePlugin::<Vec2, TComp>)
    }
}
