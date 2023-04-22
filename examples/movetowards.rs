use bevy::{log::LogPlugin, prelude::*, window::PrimaryWindow};
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
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugin(
            AutomaticUpdate::<NearestNeighbour>::new()
                .with_spatial_ds(SpatialStructure::KDTree3)
                .with_frequency(Duration::from_secs(1))
                .with_transform(TransformMode::Transform),
        )
        .add_startup_system(setup)
        .add_system(mouseclick)
        .add_system(move_to)
        .run();
}

type NNTree = KDTree3<NearestNeighbour>;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for x in -6..6 {
        for y in -6..6 {
            commands.spawn((
                NearestNeighbour,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.7, 0.3, 0.5),
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new((x * 100) as f32, (y * 100) as f32, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

fn mouseclick(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let win = window.get_single().unwrap();
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(mut pos) = win.cursor_position() {
            pos.x -= win.width() / 2.0;
            pos.y -= win.height() / 2.0;
            commands.spawn((
                MoveTowards,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.15, 0.15, 1.0),
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: pos.extend(0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
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
            transform.translation += towards.normalize() * time.delta_seconds() * 64.0;
        }
    }
}
