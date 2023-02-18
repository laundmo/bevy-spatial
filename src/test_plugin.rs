use std::marker::PhantomData;

use bevy::{math::Vec3A, prelude::*};

use crate::{
    datacontainer::{SpatialData, TComp},
    point::Point3A,
    SpatialUpdate,
};

#[derive(Default)]
pub struct TestPlugin<Comp>(PhantomData<Comp>);

impl<Comp: TComp> Plugin for TestPlugin<Comp> {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialData<Point3A, Comp>>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                extract_all::<Comp>
                    .label(SpatialUpdate::ExtractCoordinates)
                    .before(SpatialUpdate::UpdateSpatial),
            );
        // .add_system(log);
    }
}

fn extract_all<Comp: TComp>(
    data: Query<(Entity, &GlobalTransform)>,
    mut sd: ResMut<SpatialData<Point3A, Comp>>,
) {
    sd.set_all(
        data.iter()
            .map(|(e, p)| (e, (e, p.translation_vec3a()).into())),
    );
}

// fn log(sd: Res<SpatialData<Vec3A, SpatialComponent>>) {
//     dbg!(sd);
// }
