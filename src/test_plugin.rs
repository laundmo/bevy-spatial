use bevy::{math::Vec3A, prelude::*};

use crate::{datacontainer::SpatialData, point::Point3A};

pub struct TestPlugin;

#[derive(Component, Debug)]
struct SpatialComponent;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialData::<Point3A, SpatialComponent>::new())
            .add_system(extract_all);
        // .add_system(log);
    }
}

fn extract_all(
    data: Query<(Entity, &GlobalTransform)>,
    mut sd: ResMut<SpatialData<Point3A, SpatialComponent>>,
) {
    data.iter()
        .for_each(|(e, p)| sd.add_changed(e, p.translation_vec3a()));
}

// fn log(sd: Res<SpatialData<Vec3A, SpatialComponent>>) {
//     dbg!(sd);
// }
