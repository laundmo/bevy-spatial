use std::marker::PhantomData;

use crate::{
    point::{SpatialPoint, VecFromGlobalTransform, VecFromTransform},
    spatial_access::UpdateSpatialAccess,
    SpatialAccess,
};

use bevy::{ecs::schedule::FreeSystemSet, prelude::*};

/// Select which Transform to use when automatically updating the Spatial Datastructure.
#[derive(Clone, Default, Copy)]
pub enum TransformMode {
    /// Uses the normal [`Transform`] for updating the Spatial Datastructure.
    #[default]
    Transform,
    /// Uses the [`GlobalTransform`] for updating the Spatial Datastructure.
    GlobalTransform,
}
// long term todo: add support for CustomCoordinate mode which uses a user-defined type implementing a trait like VecFromTransform

type GlamVec<S> = <<S as SpatialAccess>::Point as SpatialPoint>::Vec;

pub(crate) struct AutoT<SpatialDS>(PhantomData<SpatialDS>);

impl<SpatialDS> AutoT<SpatialDS>
where
    GlamVec<SpatialDS>: VecFromTransform,
    SpatialDS: UpdateSpatialAccess + Resource,
    <SpatialDS as SpatialAccess>::Point: From<(Entity, GlamVec<SpatialDS>)>,
    SpatialDS::Comp: Component,
{
    #[allow(clippy::needless_pass_by_value)]
    fn update_ds(
        mut tree: ResMut<SpatialDS>,
        changed: Query<(Entity, Ref<Transform>), With<SpatialDS::Comp>>,
        mut removed: RemovedComponents<SpatialDS::Comp>,
    ) {
        tree.update(
            changed.iter().map(|(e, ch)| {
                let changed = ch.is_changed();
                (
                    (e, GlamVec::<SpatialDS>::from_transform(ch.into_inner())).into(),
                    changed,
                )
            }),
            removed.iter(),
        );
    }

    pub fn build(app: &mut App, set: impl FreeSystemSet) {
        app.add_system(Self::update_ds.no_default_base_set().in_set(set));
    }
}

pub(crate) struct AutoGT<SpatialDS>(PhantomData<SpatialDS>);

impl<SpatialDS> AutoGT<SpatialDS>
where
    GlamVec<SpatialDS>: VecFromGlobalTransform,
    SpatialDS: UpdateSpatialAccess + Resource,
    <SpatialDS as SpatialAccess>::Point: From<(Entity, GlamVec<SpatialDS>)>,
    SpatialDS::Comp: Component,
{
    #[allow(clippy::needless_pass_by_value)]
    fn update_ds(
        mut tree: ResMut<SpatialDS>,
        changed: Query<(Entity, Ref<GlobalTransform>), With<SpatialDS::Comp>>,
        mut removed: RemovedComponents<SpatialDS::Comp>,
    ) {
        tree.update(
            changed.iter().map(|(e, ch)| {
                let changed = ch.is_changed();
                (
                    (e, GlamVec::<SpatialDS>::from_transform(ch.into_inner())).into(),
                    changed,
                )
            }),
            removed.iter(),
        );
    }

    pub fn build(app: &mut App, set: impl FreeSystemSet) {
        app.add_system(Self::update_ds.no_default_base_set().in_set(set));
    }
}
