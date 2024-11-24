use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec2,
    prelude::*,
    time::common_conditions::on_timer,
    window::PrimaryWindow,
};
use bevy_spatial::{
    kdtree::KDTree3, AutomaticUpdate, SpatialAccess, SpatialStructure, TransformMode,
};
use std::time::Duration;

#[derive(Component, Default)]
struct NearestNeighbour;

#[derive(Component)]
struct MoveTowards;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            AutomaticUpdate::<NearestNeighbour>::new()
                .with_spatial_ds(SpatialStructure::KDTree3)
                .with_frequency(Duration::from_secs(1))
                .with_transform(TransformMode::Transform),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                mouseclick,
                log_num.run_if(on_timer(Duration::from_secs_f32(0.5))),
                move_to,
            ),
        )
        .run();
}

type NNTree = KDTree3<NearestNeighbour>;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    for x in -6..6 {
        for y in -6..6 {
            commands.spawn((
                NearestNeighbour,
                Sprite {
                    color: Color::srgb(0.7, 0.3, 0.5),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new((x * 100) as f32, (y * 100) as f32, 0.0),
                    ..default()
                },
            ));
        }
    }
}

fn mouseclick(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform)>,
) {
    let win = window.single();
    let (cam, cam_t) = cam.single();
    if mouse_input.pressed(MouseButton::Left) {
        if let Some(pos) = win.cursor_position() {
            for xoff in -1..=1 {
                for yoff in -1..=1 {
                    commands.spawn((
                        MoveTowards,
                        Sprite {
                            color: Color::srgb(0.15, 0.15, 1.0),
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..default()
                        },
                        Transform {
                            translation: cam
                                .viewport_to_world_2d(
                                    cam_t,
                                    pos + vec2(xoff as f32 * 16., yoff as f32 * 16.),
                                )
                                .unwrap_or(Vec2::ZERO)
                                .extend(1.0),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

fn move_to(
    treeaccess: Res<NNTree>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MoveTowards>>,
) {
    for mut transform in &mut query {
        if let Some(nearest) = treeaccess.nearest_neighbour(transform.translation) {
            let towards = nearest.0 - transform.translation;
            transform.translation += towards.normalize() * time.delta_secs() * 64.0;
        }
    }
}

fn log_num(query: Query<(), With<MoveTowards>>) {
    info!(target: "blue", amount = query.iter().len())
}
