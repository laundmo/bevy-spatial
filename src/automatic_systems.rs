use std::marker::PhantomData;

use crate::{
    point::{SpatialPoint, VecFromGlobalTransform, VecFromTransform},
    spatial_access::UpdateSpatialAccess,
    SpatialAccess,
};

use bevy::{ecs::schedule::FreeSystemSet, prelude::*};

/// Select which Transform to use when updating the Spatial Datastructure.
///
/// Only applies when using [`update_automatic_with`](super::plugin::SpatialPlugin::update_automatic_with) mode.
#[derive(Clone, Default, Copy)]
pub enum TransformMode {
    /// Uses the normal [`Transform`] for updating the Spatial Datastructure.
    #[default]
    Transform,
    /// Uses the [`GlobalTransform`] for updating the Spatial Datastructure.
    GlobalTransform,
}

type GlamVec<S> = <<S as SpatialAccess>::Point as SpatialPoint>::Vec;

pub struct AutoUpdateTransform<SpatialDS>(PhantomData<SpatialDS>);

impl<SpatialDS> AutoUpdateTransform<SpatialDS>
where
    GlamVec<SpatialDS>: VecFromTransform,
    SpatialDS: UpdateSpatialAccess + Resource,
    <SpatialDS as SpatialAccess>::Point: From<(Entity, GlamVec<SpatialDS>)>,
    SpatialDS::Comp: Component,
{
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

pub struct AutoUpdateGTransform<SpatialDS>(PhantomData<SpatialDS>);

impl<SpatialDS> AutoUpdateGTransform<SpatialDS>
where
    GlamVec<SpatialDS>: VecFromGlobalTransform,
    SpatialDS: UpdateSpatialAccess + Resource,
    <SpatialDS as SpatialAccess>::Point: From<(Entity, GlamVec<SpatialDS>)>,
    SpatialDS::Comp: Component,
{
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
