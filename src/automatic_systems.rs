use super::{plugin::SpatialStructure, point::SpatialTracker, point::VecFromTransform, TComp};
use bevy::{
    math::{DVec2, DVec3, Vec3A},
    prelude::*,
};

macro_rules! impl_automatic_systems {
    ($fnname:ident, $bvec:ty, $tr:ty) => {
        fn $fnname<Comp: TComp, P: VecFromTransform<$tr, Vec = $bvec>>(
            mut commands: Commands,
            added_q: Query<(Entity, &$tr), Added<Comp>>,
            mut tracker_q: Query<&mut SpatialTracker<Comp, $bvec>>,
        ) {
            for (e, t) in &added_q {
                let track = tracker_q.get_mut(e);
                let vec = P::from_transform(t);
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

impl_automatic_systems!(sys, Vec2, Transform);

#[derive(Clone, Default, Copy)]
pub enum TransformMode {
    #[default]
    Transform,
    GlobalTransform,
}

// fn add_by_modes<Comp: TComp>(app: &mut App, tmode: TransformMode, structure: SpatialStructure) {
//     match tmode {
//         TransformMode::Transform => match structure {
//             SpatialStructure::KDTree2 => {
//                 app.add_system(vec2_transform_added::<Comp>);
//             }
//             SpatialStructure::KDTree3 => {
//                 app.add_system(vec3_transform_added::<Comp>);
//             }
//             SpatialStructure::KDTree3A => {
//                 app.add_system(vec3a_transform_added::<Comp>);
//             }
//             SpatialStructure::KDTreeD2 => {
//                 app.add_system(dvec2_transform_added::<Comp>);
//             }
//             SpatialStructure::KDTreeD3 => {
//                 app.add_system(dvec3_transform_added::<Comp>);
//             }
//         },
//         TransformMode::GlobalTransform => match structure {
//             SpatialStructure::KDTree2 => {
//                 app.add_system(vec2_gtransform_added::<Comp>);
//             }
//             SpatialStructure::KDTree3 => {
//                 app.add_system(vec3_gtransform_added::<Comp>);
//             }
//             SpatialStructure::KDTree3A => {
//                 app.add_system(vec3a_gtransform_added::<Comp>);
//             }
//             SpatialStructure::KDTreeD2 => {
//                 app.add_system(dvec2_gtransform_added::<Comp>);
//             }
//             SpatialStructure::KDTreeD3 => {
//                 app.add_system(dvec3_gtransform_added::<Comp>);
//             }
//         },
//     }
// }
