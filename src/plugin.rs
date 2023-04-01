use std::{marker::PhantomData, time::Duration};

use bevy::{ecs::schedule::FreeSystemSet, prelude::*};

use crate::{
    automatic_systems::{automatic_systems, TransformMode},
    kdtree::{KDTree2, KDTree3, KDTree3A},
    timestep::{on_timer_changeable, TimestepLength},
    TComp,
};

/// Default set for spatial datastructure updates. Can be overridden using [`SpatialPlugin::in_set`]
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SpatialSet;

#[derive(Copy, Clone, Default)]
pub enum SpatialStructure {
    /// Corresponds to [`KdTree2`](crate::kdtree::KDTree2)
    KDTree2,
    /// Corresponds to [`KdTree3`](crate::kdtree::KDTree3)
    #[default]
    KDTree3,
    /// Corresponds to [`KdTree3A`](crate::kdtree::KDTree3A)
    KDTree3A,
    // Linear/naive (linfa?)
    // Grid
    // RStar
}

impl SpatialStructure {
    fn init_tree<'a, Comp: TComp>(&'a self, app: &'a mut App, mode: &'a UpdateMode) -> &mut App {
        match *self {
            SpatialStructure::KDTree2 => <KDTree2<Comp>>::build(mode, app),
            SpatialStructure::KDTree3 => <KDTree3<Comp>>::build(mode, app),
            SpatialStructure::KDTree3A => <KDTree3A<Comp>>::build(mode, app),
        }
    }
}

#[derive(Clone)]
pub(crate) enum UpdateMode {
    /// Update from the [`SpatialTracker`](crate::point::SpatialTracker) when a event is sent.
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

pub struct SpatialPlugin<Comp, Set>
where
    Comp: TComp,
    Set: FreeSystemSet,
{
    pub(crate) comp: PhantomData<Comp>,
    pub(crate) set: Set,
    pub(crate) spatial_structure: SpatialStructure,
    pub(crate) update_mode: UpdateMode,
}

/// Builds a [`SpatialPlugin`].
///
/// Exists only to avoid needing to specify the [`FreeSystemSet`] manually.
/// If you want to change the set from the default, use [`in_set`](SpatialPlugin::in_set)
pub struct SpatialBuilder;

impl SpatialBuilder {
    /// Make a new [`SpatialPlugin`] with the default values.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<Comp: TComp>() -> SpatialPlugin<Comp, SpatialSet> {
        default()
    }
}

impl<Comp: TComp> Default for SpatialPlugin<Comp, SpatialSet> {
    fn default() -> Self {
        SpatialPlugin {
            comp: PhantomData,
            set: SpatialSet,
            update_mode: default(),
            spatial_structure: default(),
        }
    }
}

impl<Comp: TComp, Set: FreeSystemSet> SpatialPlugin<Comp, Set> {
    /// Which spatial structure to use.
    ///
    /// Available options are:
    ///
    /// - [`SpatialStructure::KDTree2`] stores [`Vec2`]
    /// - [`SpatialStructure::KDTree3`] stores [`Vec3`]
    /// - [`SpatialStructure::KDTree3A`] stores [`Vec3A`](bevy::math::Vec3A)
    pub fn spatial_structure(mut self, spatial_structure: SpatialStructure) -> Self {
        self.spatial_structure = spatial_structure;
        self
    }

    /// Change the Bevy [`FreeSystemSet`] in which this plugin will put its systems.
    pub fn in_set<NewSet: FreeSystemSet>(self, set: NewSet) -> SpatialPlugin<Comp, NewSet> {
        // Struct filling for differing types is experimental. Have to manually list each.
        SpatialPlugin {
            set,
            comp: PhantomData,
            spatial_structure: self.spatial_structure,
            update_mode: self.update_mode,
        }
    }

    /// Automatically update the spatial data structure every duration,
    /// using either [`Transform`] or [`GlobalTransform`] based on [`TransformMode`].
    ///
    /// This is essentially the same as [`update_from_tracker`](SpatialPlugin::update_from_tracker).,
    /// with a added automatic system to extract the coordinate data to the [`SpatialTracker`](crate::point::SpatialTracker) and a already-set-up timer.
    ///
    /// The systems added by this are added to the [`set`](SpatialPlugin::in_set).
    pub fn update_automatic_with(mut self, duration: Duration, mode: TransformMode) -> Self {
        self.update_mode = UpdateMode::AutomaticTimer(duration, mode);
        self
    }

    /// Update based on the data in [`SpatialTracker`](crate::point::SpatialTracker), whenever a [`UpdateEvent`] is sent.
    ///
    /// In this mode, you are supposed to write whatever data you want to [`SpatialTracker`](crate::point::SpatialTracker).
    /// This allows full control over the coordinates used. Great for custom coordinate systems.
    ///
    /// The systems added by this are added to the [`set`](SpatialPlugin::in_set).
    pub fn update_from_tracker(mut self) -> Self {
        self.update_mode = UpdateMode::FromTracker;
        self
    }

    /// For fully manual updates. See todo!("manual updates example") for a example on how to do this.
    ///
    /// Does nothing except adding the Spatial Datastructure as a resource, for you to update yourself.
    /// Good if you want more control than [`update_from_tracker`](SpatialPlugin::update_from_tracker) allows for.
    pub fn update_manual(mut self) -> Self {
        self.update_mode = UpdateMode::Manual;
        self
    }
}

impl<Comp: TComp, Set: FreeSystemSet + Copy> Plugin for SpatialPlugin<Comp, Set> {
    fn build(&self, app: &mut App) {
        self.spatial_structure
            .init_tree::<Comp>(app, &self.update_mode);
        match self.update_mode {
            UpdateMode::FromTracker => {
                app.configure_set(self.set);
            }
            UpdateMode::AutomaticTimer(ref timer, mode) => {
                automatic_systems::<Comp>(app, mode, self.spatial_structure, self.set);
                app.insert_resource(TimestepLength(*timer, PhantomData::<Comp>))
                    .configure_set(self.set.run_if(on_timer_changeable::<Comp>));
            }
            UpdateMode::Manual => (),
        }
    }
}

/// Event used to signal to a Spatial Datastructure that it should update from the [`SpatialTracker`](crate::point::SpatialTracker).
#[derive(Default, Copy, Clone)]
pub struct UpdateEvent<SpatialStructure>(PhantomData<SpatialStructure>);

pub fn send_update_event<SpatialStructure: Send + Sync + 'static>(
    mut evnt: EventWriter<UpdateEvent<SpatialStructure>>,
) {
    evnt.send(UpdateEvent(PhantomData));
}
