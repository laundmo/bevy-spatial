use std::{any::Any, marker::PhantomData, time::Duration};

use bevy::{
    app::PluginGroupBuilder,
    ecs::schedule::FreeSystemSet,
    math::{DVec2, DVec3, Vec3A},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::{
    automatic_systems::TransformMode,
    point::SpatialTracker,
    spatial_access::SpatialAccess,
    timestep::{on_timer_changeable, TimestepLength},
    KDTree2, KDTree3, KDTree3A, KDTreeD2, KDTreeD3, KDTreePlugin3A, TComp,
};

pub struct Spatial;

/// Default set for spatial datastructure updates. Can be overridden using [`SpatialPluginBuilder::in_set`]
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SpatialSet;

impl Spatial {
    pub fn new<Comp: TComp>() -> SpatialPluginBuilder<Comp, SpatialSet> {
        SpatialPluginBuilder {
            comp: PhantomData,
            set: SpatialSet,
            update_mode: default(),
            spatial_structure: default(),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub enum SpatialStructure {
    KDTree2,
    KDTree3,
    #[default]
    KDTree3A,
    // Linear/naive (linfa?)
    // Grid
    // RStar
}

impl SpatialStructure {
    fn init_tree<'a, Comp: TComp>(&'a self, app: &'a mut App) -> &mut App {
        todo!("move resource into plugin and use plugin here instead");
        match *self {
            SpatialStructure::KDTree2 => app.init_resource::<KDTree2<Comp>>(),
            SpatialStructure::KDTree3 => app.init_resource::<KDTree3<Comp>>(),
            SpatialStructure::KDTree3A => app.init_resource::<KDTree3A<Comp>>(),
        }
    }
}

#[derive(Clone)]
pub enum UpdateMode {
    /// Update from the [`SpatialTracker`] - based on the event sent.
    FromTracker,
    /// Automatically update based on changed trackers (Added, Removed) with a timer.
    /// Use this to get the least effort setup.
    AutomaticTimer(Duration, TransformMode),
    Manual,
}

impl Default for UpdateMode {
    /// 50 ms means 20 times per second. That should be fine as a default.
    fn default() -> Self {
        UpdateMode::AutomaticTimer(Duration::from_millis(50), TransformMode::Transform)
    }
}

pub struct SpatialPluginBuilder<Comp, Set>
where
    Comp: TComp,
    Set: FreeSystemSet,
{
    pub comp: PhantomData<Comp>,
    pub set: Set,
    pub spatial_structure: SpatialStructure,
    pub update_mode: UpdateMode,
}

impl<Comp: TComp, Set: FreeSystemSet> SpatialPluginBuilder<Comp, Set> {
    pub fn in_set<NewSet: FreeSystemSet>(self, set: NewSet) -> SpatialPluginBuilder<Comp, NewSet> {
        // Struct filling for differing types is experimental. Have to manually list each.
        SpatialPluginBuilder {
            set,
            comp: PhantomData,
            spatial_structure: self.spatial_structure,
            update_mode: self.update_mode,
        }
    }

    /// Automatically update the spatial data structure every duration,
    /// using either [`Transform`] or [`GlobalTransform`] based on [`TransformMode`].
    ///
    /// This is essentially the same as [`SpatialPluginBuilder::update_from_tracker`],
    /// with a added automatic system to extract the coordinate data and a already-set-up timer.
    ///
    /// The systems added by this are added to the [`SpatialPluginBuilder::set`].
    pub fn update_automatic_with(mut self, duration: Duration, mode: TransformMode) -> Self {
        self.update_mode = UpdateMode::AutomaticTimer(duration, mode);
        self
    }

    /// Update based on the data in [`SpatialTracker`], whenever a [`UpdateEvent`] is sent.
    ///
    /// The systems added by this are added to the [`SpatialPluginBuilder::set`].
    pub fn update_from_tracker(mut self) -> Self {
        self.update_mode = UpdateMode::FromTracker;
        self
    }

    /// For fully manual updates
    pub fn update_manual(mut self) -> Self {
        self.update_mode = UpdateMode::Manual;
        self
    }
}

impl<Comp: TComp, Set: FreeSystemSet + Copy> Plugin for SpatialPluginBuilder<Comp, Set> {
    fn build(&self, app: &mut App) {
        self.spatial_structure.init_tree::<Comp>(app);
        match self.update_mode {
            UpdateMode::FromTracker => {
                app.configure_set(self.set);
            }
            UpdateMode::AutomaticTimer(ref timer, mode) => {
                app.insert_resource(TimestepLength(*timer, PhantomData::<Comp>))
                    .configure_set(self.set.run_if(on_timer_changeable::<Comp>));
                todo!("Add systems to update SpatialTracker component automatically from Transform or GlobalTransform.");
            }
            UpdateMode::Manual => (),
        }
    }
}

/// Event used to signal to a Spatial Datastructure that it should update from the [`SpatialTracker`].
pub struct UpdateEvent<Comp: TComp, T>(PhantomData<Comp>, PhantomData<T>);
