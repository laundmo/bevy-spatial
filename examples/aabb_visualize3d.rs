use bevy::prelude::*;
use bevy_spatial::{CubeAABB, DebugAABB, AABB};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugAABB::<Debug>::default())
        .add_startup_system(setup)
        .add_system(mouse)
        .run();
}

#[derive(Component)]
struct Center;

#[derive(Component)]
struct Cursor;

#[derive(Component)]
struct Debug;

#[derive(Clone)]
struct MaterialHandles {
    black: Handle<StandardMaterial>,
    blue: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handles = MaterialHandles {
        black: materials.add(Color::BLACK.into()),
        blue: materials.add(Color::BLUE.into()),
    };
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 100.0, 900.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: handles.blue.clone(),
            transform: Transform::from_translation(Vec3::splat(0.0)).with_scale(Vec3::splat(10.0)),
            ..default()
        })
        .insert(Debug)
        .insert(Center);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: handles.blue.clone(),
            transform: Transform::from_translation(Vec3::splat(20.0)).with_scale(Vec3::splat(20.0)),
            ..default()
        })
        .insert(Center)
        .insert(Debug);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: handles.black.clone(),
            transform: Transform::from_translation(Vec3::splat(20.0)).with_scale(Vec3::splat(30.0)),
            ..default()
        })
        .insert(Cursor)
        .insert(Debug);
}

fn mouse(windows: Res<Windows>, mut query: Query<&mut Transform, With<Cursor>>) {
    let win = windows.get_primary().unwrap();
    if let Some(mut pos) = win.cursor_position() {
        pos.x -= win.width() / 2.0;
        pos.y -= win.height() / 2.0;
        for mut transform in query.iter_mut() {
            transform.translation = pos.extend(0.0);
        }
    }
}
