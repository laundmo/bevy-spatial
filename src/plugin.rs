use std::{any::Any, marker::PhantomData, time::Duration};

use bevy::{
    app::PluginGroupBuilder, ecs::schedule::FreeSystemSet, prelude::*,
    time::common_conditions::on_timer,
};

use crate::{
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
            base_set: CoreSet::PostUpdate,
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
    KDTreeD2,
    KDTreeD3,
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
            SpatialStructure::KDTreeD2 => app.init_resource::<KDTreeD2<Comp>>(),
            SpatialStructure::KDTreeD3 => app.init_resource::<KDTreeD3<Comp>>(),
        }
    }
}

#[derive(Clone, Default)]
pub enum UpdateMode {
    #[default]
    FromTracker,
    AutomaticTimer(Duration),
    Manual,
}

pub struct SpatialPluginBuilder<Comp, Set>
where
    Comp: TComp,
    Set: FreeSystemSet,
{
    pub comp: PhantomData<Comp>,
    pub base_set: CoreSet,
    pub set: Set,
    pub spatial_structure: SpatialStructure,
    pub update_mode: UpdateMode,
}

impl<Comp: TComp, Set: FreeSystemSet> SpatialPluginBuilder<Comp, Set> {
    pub fn in_core_set(mut self, core_set: CoreSet) -> Self {
        self.base_set = core_set;
        self
    }

    pub fn in_set<NewSet: FreeSystemSet>(self, set: NewSet) -> SpatialPluginBuilder<Comp, NewSet> {
        // Struct filling for differing types is experimental. Have to manually list each.
        SpatialPluginBuilder {
            set,
            comp: PhantomData,
            base_set: self.base_set,
            spatial_structure: self.spatial_structure,
            update_mode: self.update_mode,
        }
    }

    pub fn automatic_with_timestep(mut self, duration: Duration) -> Self {
        self.update_mode = UpdateMode::AutomaticTimer(duration);
        todo!("use typestate for this");
        assert!(self.set.type_id() == SpatialSet.type_id());
        self
    }
}

impl<Comp: TComp, Set: FreeSystemSet + Copy> Plugin for SpatialPluginBuilder<Comp, Set> {
    fn build(&self, app: &mut App) {
        self.spatial_structure.init_tree::<Comp>(app);
        match self.update_mode {
            UpdateMode::FromTracker => {
                app.configure_set(self.set.in_base_set(self.base_set.clone()));
            }
            UpdateMode::AutomaticTimer(ref timer) => {
                todo!("Add systems to update SpatialTracker component automatically from Transform or GlobalTransform.");
                app.insert_resource(TimestepLength(*timer, PhantomData::<Comp>))
                    .configure_set(
                        SpatialSet
                            .in_base_set(self.base_set.clone())
                            .run_if(on_timer_changeable::<Comp>),
                    );
            }
            UpdateMode::Manual => (),
        }
    }
}
