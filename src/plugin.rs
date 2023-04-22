use std::{marker::PhantomData, time::Duration};

use bevy::{ecs::schedule::FreeSystemSet, prelude::*};

use crate::{
    automatic_systems::{AutoGT, AutoT, TransformMode},
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

pub struct AutomaticUpdate<Comp>(PhantomData<Comp>);

impl<Comp: TComp> AutomaticUpdate<Comp> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AutomaticUpdatePlugin<Comp, SpatialSet> {
        default()
    }
}

pub struct AutomaticUpdatePlugin<Comp, Set>
where
    Set: FreeSystemSet,
{
    pub(crate) comp: PhantomData<Comp>,
    pub(crate) set: Set,
    pub(crate) frequency: Duration,
    pub(crate) transform: TransformMode,
    pub(crate) spatial_ds: SpatialStructure,
}

impl<Comp: TComp> Default for AutomaticUpdatePlugin<Comp, SpatialSet> {
    fn default() -> Self {
        AutomaticUpdatePlugin {
            comp: PhantomData,
            set: SpatialSet,
            frequency: Duration::from_millis(50),
            transform: TransformMode::Transform,
            spatial_ds: default(),
        }
    }
}

impl<Tree, Set: FreeSystemSet> AutomaticUpdatePlugin<Tree, Set> {
    /// Change the Bevy [`FreeSystemSet`] in which this plugin will put its systems.
    pub fn with_set<NewSet: FreeSystemSet>(
        self,
        set: NewSet,
    ) -> AutomaticUpdatePlugin<Tree, NewSet> {
        // Struct filling for differing types is experimental. Have to manually list each.
        AutomaticUpdatePlugin::<Tree, NewSet> {
            set,
            comp: PhantomData,
            frequency: self.frequency,
            transform: self.transform,
            spatial_ds: self.spatial_ds,
        }
    }

    /// Change which spatial datastructure is used.
    ///
    ///
    pub fn with_spatial_ds(self, spatial_ds: SpatialStructure) -> Self {
        Self { spatial_ds, ..self }
    }

    pub fn with_frequency(self, frequency: Duration) -> Self {
        Self { frequency, ..self }
    }

    pub fn with_transform(self, transform: TransformMode) -> Self {
        Self { transform, ..self }
    }
}

impl<Comp: TComp, Set: FreeSystemSet + Copy> Plugin for AutomaticUpdatePlugin<Comp, Set> {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimestepLength(self.frequency, PhantomData::<Comp>))
            .configure_set(self.set.run_if(on_timer_changeable::<Comp>));

        match self.spatial_ds {
            SpatialStructure::KDTree2 => app.init_resource::<KDTree2<Comp>>(),
            SpatialStructure::KDTree3 => app.init_resource::<KDTree3<Comp>>(),
            SpatialStructure::KDTree3A => app.init_resource::<KDTree3A<Comp>>(),
        };

        match self.transform {
            TransformMode::Transform => match self.spatial_ds {
                SpatialStructure::KDTree2 => AutoT::<KDTree2<Comp>>::build(app, self.set),
                SpatialStructure::KDTree3 => AutoT::<KDTree3<Comp>>::build(app, self.set),
                SpatialStructure::KDTree3A => AutoT::<KDTree3A<Comp>>::build(app, self.set),
            },
            TransformMode::GlobalTransform => match self.spatial_ds {
                SpatialStructure::KDTree2 => AutoGT::<KDTree2<Comp>>::build(app, self.set),
                SpatialStructure::KDTree3 => AutoGT::<KDTree3<Comp>>::build(app, self.set),
                SpatialStructure::KDTree3A => AutoGT::<KDTree3A<Comp>>::build(app, self.set),
            },
        };
    }
}
