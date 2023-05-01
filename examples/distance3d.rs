use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_spatial::{kdtree::KDTree3, AutomaticUpdate, SpatialAccess};

#[derive(Component)]
struct NearestNeighbourComponent;

#[derive(Component)]
struct Cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AutomaticUpdate::<NearestNeighbourComponent>::new())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(mouse)
        .add_system(color)
        .add_system(reset_color.before(color))
        .run();
}

#[derive(Resource, Clone)]
struct MaterialHandles {
    orange_red: Handle<StandardMaterial>,
    black: Handle<StandardMaterial>,
    blue: Handle<StandardMaterial>,
}

type NNTree = KDTree3<NearestNeighbourComponent>;

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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 100.0, 900.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands
        .spawn(PbrBundle {
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
                    .spawn(PbrBundle {
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
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Cursor>>,
) {
    let window = window.single();
    if let Some(mut pos) = window.cursor_position() {
        pos.x -= window.width() / 2.0;
        pos.y -= window.height() / 2.0;
        let mut transform = query.single_mut();
        transform.translation = pos.extend(0.0);
    }
}

fn color(
    window: Query<&Window, With<PrimaryWindow>>,

    treeaccess: Res<NNTree>,
    mut query: Query<&mut Handle<StandardMaterial>, With<NearestNeighbourComponent>>,
    colors: Res<MaterialHandles>,
) {
    let window = window.single();
    if let Some(mut pos) = window.cursor_position() {
        pos.x -= window.width() / 2.0;
        pos.y -= window.height() / 2.0;

        for (_, entity) in treeaccess.within_distance(pos.extend(0.0), 100.0) {
            if let Ok(mut handle) = query.get_mut(entity.expect("No entity")) {
                *handle = colors.black.clone();
            }
        }
    }
}

fn reset_color(
    colors: Res<MaterialHandles>,
    mut query: Query<&mut Handle<StandardMaterial>, With<NearestNeighbourComponent>>,
) {
    for mut handle in &mut query {
        *handle = colors.orange_red.clone();
    }
}
