use std::{any::Any, marker::PhantomData, time::Duration};

use bevy::{app::PluginGroupBuilder, ecs::schedule::FreeSystemSet, prelude::*};

use crate::{
    spatial_access::SpatialAccess, timestep::on_timer_changeable, KDTree2, KDTree3, KDTree3A,
    KDTreeD2, KDTreeD3, KDTreePlugin3A, TComp,
};

pub struct Spatial;

/// Default set for spatial datastructure updates. Can be overridden using [`SpatialPluginBuilder::in_set`]
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct SpatialSet;

impl Spatial {
    fn new<Comp: TComp>() -> SpatialPluginBuilder<Comp, SpatialSet> {
        SpatialPluginBuilder {
            comp: PhantomData,
            set: SpatialSet,
            base_set: CoreSet::PostUpdate,
            update_mode: default(),
            spatial_structure: default(),
        }
    }
}

#[derive(Copy, Clone, Default)]
enum SpatialStructure {
    KDTree2,
    KDTree3,
    #[default]
    KDTree3A,
    KDTreeD2,
    KDTreeD3,
    // Linear/naive (linfa?)
    // Grid
    // RStar
}

impl SpatialStructure {
    fn init_tree<'a, Comp: TComp>(&'a self, app: &'a mut App) -> &mut App {
        match *self {
            SpatialStructure::KDTree2 => app.init_resource::<KDTree2<Comp>>(),
            SpatialStructure::KDTree3 => app.init_resource::<KDTree3<Comp>>(),
            SpatialStructure::KDTree3A => app.init_resource::<KDTree3A<Comp>>(),
            SpatialStructure::KDTreeD2 => app.init_resource::<KDTreeD2<Comp>>(),
            SpatialStructure::KDTreeD3 => app.init_resource::<KDTreeD3<Comp>>(),
        }
    }
}

#[derive(Clone, Default)]
enum UpdateMode {
    #[default]
    Automatic,
    AutomaticTimer(Timer),
    Manual,
}

pub struct SpatialPluginBuilder<Comp, Set>
where
    Comp: TComp,
    Set: FreeSystemSet,
{
    comp: PhantomData<Comp>,
    base_set: CoreSet,
    set: Set,
    spatial_structure: SpatialStructure,
    update_mode: UpdateMode,
}

impl<Comp: TComp, Set: FreeSystemSet> SpatialPluginBuilder<Comp, Set> {
    fn in_core_set(mut self, core_set: CoreSet) -> Self {
        self.base_set = core_set;
        self
    }

    fn in_set<NewSet: FreeSystemSet>(self, set: NewSet) -> SpatialPluginBuilder<Comp, NewSet> {
        // Struct filling for differing types is experimental. Have to manually list each.
        SpatialPluginBuilder {
            set,
            comp: PhantomData,
            base_set: self.base_set,
            spatial_structure: self.spatial_structure,
            update_mode: self.update_mode,
        }
    }

    fn automatic_with_timestep(mut self, duration: Duration) {
        self.update_mode = UpdateMode::AutomaticTimer(Timer::new(duration, TimerMode::Repeating));
    }
}

impl<Comp: TComp, Set: FreeSystemSet + Copy> Plugin for SpatialPluginBuilder<Comp, Set> {
    fn build(&self, app: &mut App) {
        self.spatial_structure.init_tree::<Comp>(app);
        match self.update_mode {
            UpdateMode::Automatic | UpdateMode::AutomaticTimer(_) => {
                app.configure_set(self.set.in_base_set(self.base_set.clone()))
                    .add_systems((test,).in_set(self.set));
            }
            UpdateMode::Manual => (),
        }
    }
}

fn test() {}
