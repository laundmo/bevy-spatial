use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Vec3Swizzles,
    prelude::*,
};
use bevy_spatial::{EfficientInsertParams, RTreeAccess3D, RTreePlugin3D, SpatialAccess};

#[derive(Component)]
struct NearestNeighbourComponent;

#[derive(Component)]
struct Cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(
            RTreePlugin3D::<NearestNeighbourComponent, EfficientInsertParams> { ..default() },
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(mouse)
        .add_system(color)
        .add_system(reset_color.before(color))
        .run();
}

#[derive(Clone)]
struct MaterialHandles {
    orange_red: Handle<StandardMaterial>,
    black: Handle<StandardMaterial>,
    blue: Handle<StandardMaterial>,
}

type NNTree = RTreeAccess3D<NearestNeighbourComponent, EfficientInsertParams>;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handles = MaterialHandles {
        orange_red: materials.add(Color::ORANGE_RED.into()),
        black: materials.add(Color::BLACK.into()),
        blue: materials.add(Color::BLUE.into()),
    };
    commands.insert_resource(handles.clone());
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 100.0, 900.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: handles.blue.clone(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Cursor);

    for x in -20..20 {
        for y in -20..20 {
            for z in -6..6 {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
                        material: handles.orange_red.clone(),
                        transform: Transform::from_xyz(
                            (x * 15) as f32,
                            (y * 15) as f32,
                            (z * 15) as f32,
                        ),
                        ..default()
                    })
                    .insert(NearestNeighbourComponent);
            }
        }
    }
}

fn mouse(
    windows: Res<Windows>,
    treeaccess: Res<NNTree>,
    mut query: Query<&mut Transform, With<Cursor>>,
) {
    let win = windows.get_primary().unwrap();
    if let Some(mut pos) = win.cursor_position() {
        pos.x = pos.x - win.width() / 2.0;
        pos.y = pos.y - win.height() / 2.0;
        let mut transform = query.single_mut();
        transform.translation = pos.extend(0.0);
        //if let Some(nearest) = treeaccess.nearest_neighbour(pos.extend(0.0)) {
        //}
    }
}

fn color(
    windows: Res<Windows>,
    treeaccess: Res<NNTree>,
    mut query: Query<&mut Handle<StandardMaterial>, With<NearestNeighbourComponent>>,
    colors: Res<MaterialHandles>,
) {
    let win = windows.get_primary().unwrap();
    if let Some(mut pos) = win.cursor_position() {
        pos.x = pos.x - win.width() / 2.0;
        pos.y = pos.y - win.height() / 2.0;

        for (_, entity) in treeaccess.within_distance(pos.extend(0.0), 100.0) {
            let mut handle = query.get_mut(entity).unwrap();
            *handle = colors.black.clone();
        }
    }
}

fn reset_color(
    colors: Res<MaterialHandles>,
    mut query: Query<&mut Handle<StandardMaterial>, With<NearestNeighbourComponent>>,
) {
    for mut handle in query.iter_mut() {
        *handle = colors.orange_red.clone();
    }
}
