use crate::{kdtree::KDTree2, plugin::send_update_event};

use super::{plugin::SpatialStructure, point::SpatialTracker, TComp};
use bevy::{ecs::schedule::FreeSystemSet, math::Vec3A, prelude::*};

#[rustfmt::skip]
mod from_transform {
    use bevy::{math::Vec3A, prelude::*};

    // Helper trait for automatic mode
    pub(super) trait VecFromTransform<T>{
        type Vec: Copy + Sized;
        fn from_transform(t: &T) -> Self::Vec;
    }

    macro_rules! impl_from_transform {
        ($vec:ident, $tr:ident, $conv:expr) => {
            impl VecFromTransform<$tr> for $vec {
                type Vec = $vec;
                fn from_transform(t: &$tr) -> Self::Vec {
                    $conv(t)
                }
            }
        };
    }

    // Transform
    impl_from_transform!(Vec2, Transform, |t: &Transform| {t.translation.truncate()});
    impl_from_transform!(Vec3, Transform, |t: &Transform| { t.translation });
    impl_from_transform!(Vec3A, Transform, |t: &Transform| { t.translation.into() });

    // GlobalTransform
    impl_from_transform!(Vec2, GlobalTransform, |t: &GlobalTransform| {t.translation().truncate()});
    impl_from_transform!(Vec3, GlobalTransform, |t: &GlobalTransform| {t.translation()});
    impl_from_transform!(Vec3A, GlobalTransform, |t: &GlobalTransform| {t.translation().into()});
}
use from_transform::VecFromTransform;

macro_rules! impl_automatic_systems {
    ($fnname:ident, $bvec:ty, $tr:ty) => {
        fn $fnname<Comp: TComp>(
            mut commands: Commands,
            added_q: Query<(Entity, &$tr), Changed<Comp>>,
            mut tracker_q: Query<&mut SpatialTracker<Comp, $bvec>>,
        ) {
            for (e, t) in &added_q {
                let track = tracker_q.get_mut(e);
                let vec = <$bvec>::from_transform(t);
                match track {
                    Ok(mut tracker) => {
                        tracker.coord = vec;
                    }
                    Err(_) => {
                        commands
                            .entity(e)
                            .insert(SpatialTracker::<Comp, $bvec>::new(vec));
                    }
                }
            }
        }
    };
}

impl_automatic_systems!(vec2_transform_changed, Vec2, Transform);
impl_automatic_systems!(vec3_transform_changed, Vec3, Transform);
impl_automatic_systems!(vec3a_transform_changed, Vec3A, Transform);

impl_automatic_systems!(vec2_gtransform_changed, Vec2, GlobalTransform);
impl_automatic_systems!(vec3_gtransform_changed, Vec3, GlobalTransform);
impl_automatic_systems!(vec3a_gtransform_changed, Vec3A, GlobalTransform);

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

// Add change tracker systems to app.
pub(crate) fn automatic_systems<Comp: TComp>(
    app: &mut App,
    tmode: TransformMode,
    structure: SpatialStructure,
    set: impl FreeSystemSet + Copy,
) {
    match tmode {
        TransformMode::Transform => match structure {
            SpatialStructure::KDTree2 => {
                app.add_systems(
                    (
                        vec2_transform_changed::<Comp>
                            .no_default_base_set()
                            .in_set(set),
                        send_update_event::<KDTree2<Comp>>,
                    )
                        .chain(),
                );
            }
            SpatialStructure::KDTree3 => {
                app.add_system(
                    vec3_transform_changed::<Comp>
                        .no_default_base_set()
                        .in_set(set),
                );
            }
            SpatialStructure::KDTree3A => {
                app.add_system(
                    vec3a_transform_changed::<Comp>
                        .no_default_base_set()
                        .in_set(set),
                );
            }
        },
        TransformMode::GlobalTransform => match structure {
            SpatialStructure::KDTree2 => {
                app.add_system(
                    vec2_gtransform_changed::<Comp>
                        .no_default_base_set()
                        .in_set(set),
                );
            }
            SpatialStructure::KDTree3 => {
                app.add_system(
                    vec3_gtransform_changed::<Comp>
                        .no_default_base_set()
                        .in_set(set),
                );
            }
            SpatialStructure::KDTree3A => {
                app.add_system(
                    vec3a_gtransform_changed::<Comp>
                        .no_default_base_set()
                        .in_set(set),
                );
            }
        },
    }
}
