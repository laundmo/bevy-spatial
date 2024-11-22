use bevy::{
    color::palettes::css as csscolors,
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
        .add_plugins(AutomaticUpdate::<NearestNeighbourComponent>::new())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(Mouse3D { pos: Vec3::ZERO })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_mouse_pos, (mouse, color, reset_color.before(color))).chain(),
        )
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
        orange_red: materials.add(Color::from(csscolors::ORANGE_RED)),
        black: materials.add(Color::from(csscolors::BLACK)),
        blue: materials.add(Color::from(csscolors::BLUE)),
    };
    commands.insert_resource(handles.clone());
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.,
    });
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 100.0, 900.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(10., 10., 10.))),
            MeshMaterial3d(handles.blue.clone()),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ))
        .insert(Cursor);

    let mesh = meshes.add(Cuboid::new(4., 4., 4.));
    for x in -25..25 {
        for y in -25..25 {
            for z in -9..9 {
                commands.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(handles.orange_red.clone()),
                    Transform::from_xyz((x * 15) as f32, (y * 15) as f32, (z * 15) as f32),
                    NearestNeighbourComponent,
                ));
            }
        }
    }
}

#[derive(Copy, Clone, Resource)]
struct Mouse3D {
    pos: Vec3,
}

fn update_mouse_pos(
    window: Query<&Window, With<PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform)>,
    mut mouse: ResMut<Mouse3D>,
) {
    let win = window.single();
    let (cam, cam_t) = cam.single();
    if let Some(w_pos) = win.cursor_position() {
        if let Ok(pos) = cam.viewport_to_world(cam_t, w_pos) {
            mouse.pos = pos.get_point(900.);
        }
    }
}

fn mouse(mouse: Res<Mouse3D>, mut query: Query<&mut Transform, With<Cursor>>) {
    let mut transform = query.single_mut();
    transform.translation = mouse.pos;
}

fn color(
    mouse: Res<Mouse3D>,
    treeaccess: Res<NNTree>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>, With<NearestNeighbourComponent>>,
    colors: Res<MaterialHandles>,
) {
    for (_, entity) in treeaccess.within_distance(mouse.pos, 100.0) {
        if let Ok(mut handle) = query.get_mut(entity.expect("No entity")) {
            *handle = colors.black.clone().into();
        }
    }
}

fn reset_color(
    colors: Res<MaterialHandles>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>, With<NearestNeighbourComponent>>,
) {
    for mut handle in &mut query {
        *handle = colors.orange_red.clone().into();
    }
}
