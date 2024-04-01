use std::{marker::PhantomData, time::Duration};

use bevy::{
    ecs::schedule::{ScheduleLabel, SystemSet},
    prelude::*,
};

use crate::{
    automatic_systems::{AutoGT, AutoT, TransformMode},
    kdtree::{KDTree2, KDTree3, KDTree3A},
    timestep::{on_timer_changeable, TimestepLength},
    TComp,
};

/// Default set for spatial datastructure updates. Can be overridden using [`AutomaticUpdate::with_set()`](crate::AutomaticUpdate)
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SpatialSet;

/// Enum containing the different types of spatial datastructure compatible with [`AutomaticUpdate`]
#[derive(Copy, Clone, Default)]
pub enum SpatialStructure {
    /// Corresponds to [`kdtree::KdTree2`](crate::kdtree::KDTree2)
    KDTree2,
    /// Corresponds to [`kdtree::KdTree3`](crate::kdtree::KDTree3)
    #[default]
    KDTree3,
    /// Corresponds to [`kdtree::KdTree3A`](crate::kdtree::KDTree3A)
    KDTree3A,
    // Linear/naive (linfa?)
    // Grid
    // RStar
}

/// Plugin struct for setting up a spatial datastructure with automatic updating.
///
///
/// ```
/// #[derive(Component, Default)]
/// struct EntityMarker;
///
/// App::new()
///    .add_plugins(DefaultPlugins)
///    .add_plugin(AutomaticUpdate::<EntityMarker>::new()
///             .with_frequency(Duration::from_secs_f32(0.3))
///             .with_spatial_ds(SpatialStructure::KDTree2)
///             .with_transform(TransformMode::GlobalTransform)
///     )
///
/// ```
pub struct AutomaticUpdate<Comp, Set = SpatialSet, Schedule = Update>
where
    Set: SystemSet,
    Schedule: ScheduleLabel + Clone,
{
    pub(crate) comp: PhantomData<Comp>,
    pub(crate) set: Set,
    pub(crate) schedule: Schedule,
    pub(crate) frequency: Duration,
    pub(crate) transform: TransformMode,
    pub(crate) spatial_ds: SpatialStructure,
}

impl<Comp, Set: SystemSet, Schedule: ScheduleLabel + Clone> AutomaticUpdate<Comp, Set, Schedule> {
    /// Create a new [`AutomaticUpdate`] with defaults. Will add to the default [`ScheduleLabel`]: [`Update`].
    #[must_use]
    pub fn new() -> AutomaticUpdate<Comp> {
        AutomaticUpdate {
            comp: PhantomData,
            set: SpatialSet,
            schedule: Update,
            frequency: Duration::from_millis(50),
            transform: TransformMode::Transform,
            spatial_ds: default(),
        }
    }

    /// Change the Bevy [`ScheduleLabel`] in which this plugin will put its systems.
    pub fn with_schedule<NewSchedule: ScheduleLabel + Clone>(
        self,
        schedule: NewSchedule,
    ) -> AutomaticUpdate<Comp, Set, NewSchedule> {
        // Struct filling for differing types is experimental. Have to manually list each.
        AutomaticUpdate {
            set: self.set,
            schedule,
            comp: PhantomData,
            frequency: self.frequency,
            transform: self.transform,
            spatial_ds: self.spatial_ds,
        }
    }

    /// Change the Bevy [`SystemSet`] in which this plugin will put its systems.
    pub fn with_set<NewSet: SystemSet + Copy>(
        self,
        set: NewSet,
    ) -> AutomaticUpdate<Comp, NewSet, Schedule> {
        // Struct filling for differing types is experimental. Have to manually list each.
        AutomaticUpdate::<Comp, NewSet, Schedule> {
            set,
            schedule: self.schedule,
            comp: PhantomData,
            frequency: self.frequency,
            transform: self.transform,
            spatial_ds: self.spatial_ds,
        }
    }

    /// Change which spatial datastructure is used.
    ///
    /// expects one of:
    /// - [`SpatialStructure::KDTree2`]
    /// - [`SpatialStructure::KDTree3`] (default)
    /// - [`SpatialStructure::KDTree3A`]
    #[must_use]
    pub fn with_spatial_ds(self, spatial_ds: SpatialStructure) -> Self {
        Self { spatial_ds, ..self }
    }

    /// Change the update rate.
    ///
    /// Expects a [Duration] which is the delay between updates.
    #[must_use]
    pub fn with_frequency(self, frequency: Duration) -> Self {
        Self { frequency, ..self }
    }

    /// Change which Transform is used to extrat coordinates from.
    ///
    /// - [`TransformMode::Transform`] (default)
    /// - [`TransformMode::GlobalTransform`]
    ///
    /// Note: using [`TransformMode::GlobalTransform`] might cause double frame-delays
    /// as Transform->GlobalTransform propagation happens in the
    /// [`TransformPropagate`](bevy::transform::TransformSystem::TransformPropagate) [`SystemSet`] in [`PostUpdate`](bevy::app::CoreSet::PostUpdate).
    /// You can order this plugins systems by modifying the default [`SpatialSet`]
    /// or using your own [`SystemSet`] by calling [`AutomaticUpdate::with_set`](Self::with_set)
    #[must_use]
    pub fn with_transform(self, transform: TransformMode) -> Self {
        Self { transform, ..self }
    }
}

impl<Comp: TComp, Set: SystemSet + Copy> Plugin for AutomaticUpdate<Comp, Set> {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimestepLength(self.frequency, PhantomData::<Comp>))
            .configure_sets(
                self.schedule.clone(),
                self.set.run_if(on_timer_changeable::<Comp>),
            );

        match self.spatial_ds {
            SpatialStructure::KDTree2 => app.init_resource::<KDTree2<Comp>>(),
            SpatialStructure::KDTree3 => app.init_resource::<KDTree3<Comp>>(),
            SpatialStructure::KDTree3A => app.init_resource::<KDTree3A<Comp>>(),
        };

        match self.transform {
            TransformMode::Transform => match self.spatial_ds {
                SpatialStructure::KDTree2 => {
                    AutoT::<KDTree2<Comp>>::build(app, self.schedule.clone(), self.set);
                }
                SpatialStructure::KDTree3 => {
                    AutoT::<KDTree3<Comp>>::build(app, self.schedule.clone(), self.set);
                }
                SpatialStructure::KDTree3A => {
                    AutoT::<KDTree3A<Comp>>::build(app, self.schedule.clone(), self.set);
                }
            },
            TransformMode::GlobalTransform => match self.spatial_ds {
                SpatialStructure::KDTree2 => {
                    AutoGT::<KDTree2<Comp>>::build(app, self.schedule.clone(), self.set);
                }
                SpatialStructure::KDTree3 => {
                    AutoGT::<KDTree3<Comp>>::build(app, self.schedule.clone(), self.set);
                }
                SpatialStructure::KDTree3A => {
                    AutoGT::<KDTree3A<Comp>>::build(app, self.schedule.clone(), self.set);
                }
            },
        };
    }
}
