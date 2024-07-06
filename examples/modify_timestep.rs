use std::time::Duration;

use bevy::color::palettes::css as csscolors;
use bevy::prelude::*;

use bevy_spatial::{
    kdtree::KDTree2, AutomaticUpdate, SpatialAccess, SpatialStructure, TimestepLength,
};

#[derive(Component, Default)]
struct NearestNeighbour;

#[derive(Component)]
struct Chaser;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            AutomaticUpdate::<NearestNeighbour>::new()
                .with_frequency(Duration::from_millis(305))
                .with_spatial_ds(SpatialStructure::KDTree2),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_to, rotate_around, mouseclick))
        .run();
}

type NNTree = KDTree2<NearestNeighbour>;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(TextBundle::from_section(
        "Click mouse to change rate",
        TextStyle {
            font_size: 30.0,
            color: Color::BLACK,
            ..default()
        },
    ));

    commands.spawn((
        Chaser,
        SpriteBundle {
            sprite: Sprite {
                color: csscolors::BLUE.into(),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
    ));

    let neighbours = [
        (csscolors::RED, Vec3::Y * 100.),
        (csscolors::RED, Vec3::NEG_Y * 100.),
        (csscolors::RED, Vec3::X * 100.),
        (csscolors::RED, Vec3::NEG_X * 100.),
    ];

    for (color, position) in neighbours {
        commands.spawn((
            NearestNeighbour,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::from(color),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
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
        if let Some(nearest) = treeaccess.nearest_neighbour(transform.translation.truncate()) {
            let towards = nearest.0.extend(0.0) - transform.translation;
            transform.translation += towards.normalize() * time.delta_seconds() * 350.0;
        }
    }
}

/// Change the timestep for
fn mouseclick(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut text: Query<&mut Text>,
    mut step: ResMut<TimestepLength<NearestNeighbour>>,
    mut other_duration: Local<Duration>,
) {
    if other_duration.is_zero() {
        *other_duration = Duration::from_millis(1);
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        let duration = step.get_duration();
        step.set_duration(*other_duration);
        text.single_mut().sections[0].value = format!(
            "Spatial Update Rate: every {}ms",
            other_duration.as_millis()
        );
        *other_duration = duration;
    }
}
