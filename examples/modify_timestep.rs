use std::time::Duration;

use bevy::prelude::*;
use bevy_spatial::{Spatial, TimestepLength};

#[derive(Component, Default)]
struct NearestNeighbour;

#[derive(Component)]
struct Chaser;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(
            Spatial::new::<NearestNeighbour>()
                .automatic_with_timestep(Duration::from_secs_f32(0.3)),
        )
        .add_startup_system(setup)
        .add_system(move_to)
        .add_system(rotate_around)
        .add_system(mouseclick)
        .run();
}

type NNTree = KDTreeAccess2D<NearestNeighbour>;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Chaser,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
    ));

    let neighbours = [
        (Color::RED, Vec3::Y * 100.),
        (Color::RED, Vec3::NEG_Y * 100.),
        (Color::RED, Vec3::X * 100.),
        (Color::RED, Vec3::NEG_X * 100.),
    ];

    for (color, position) in neighbours.iter() {
        commands.spawn((
            NearestNeighbour,
            SpriteBundle {
                sprite: Sprite {
                    color: *color,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(*position),
                ..default()
            },
        ));
    }
}

fn rotate_around(mut query: Query<&mut Transform, With<NearestNeighbour>>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Z, 1.0f32.to_radians()),
        );
    }
}

fn move_to(
    treeaccess: Res<NNTree>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Chaser>>,
) {
    for mut transform in &mut query {
        if let Some(nearest) = treeaccess.nearest_neighbour(transform.translation) {
            let towards = nearest.0 - transform.translation;
            transform.translation += towards.normalize() * time.delta_seconds() * 350.0;
        }
    }
}

fn mouseclick(
    mouse_input: Res<Input<MouseButton>>,
    mut step: ResMut<TimestepLength<NearestNeighbour>>,
    mut other_duration: Local<Duration>,
) {
    if other_duration.is_zero() {
        *other_duration = Duration::from_millis(1);
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        let duration = step.get_duration();
        step.set_duration(*other_duration);
        *other_duration = duration;
    }
}
