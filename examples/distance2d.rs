use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Vec3Swizzles,
    prelude::*,
};
use bevy_spatial::{KDTreeAccess2D, KDTreePlugin2D, SpatialAccess};

// marker for entities tracked by the KDTree
#[derive(Component)]
struct NearestNeighbourComponent;

// marker for the "cursor" entity
#[derive(Component)]
struct Cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin, which takes the tracked component as a generic.
        .add_plugin(KDTreePlugin2D::<NearestNeighbourComponent> { ..default() })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(mouse)
        .add_system(color)
        .add_system(reset_color.before(color))
        .add_system(collide_wall)
        .add_system(movement)
        .run();
}

// type alias for easier usage later
type NNTree = KDTreeAccess2D<NearestNeighbourComponent>;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn((
        Cursor,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            ..default()
        },
    ));
    let sprite = Sprite {
        color: Color::ORANGE_RED,
        custom_size: Some(Vec2::new(6.0, 6.0)),
        ..default()
    };
    for x in -100..100 {
        for y in -100..100 {
            commands.spawn((
                NearestNeighbourComponent,
                SpriteBundle {
                    sprite: sprite.clone(),
                    transform: Transform {
                        translation: Vec3::new((x * 4) as f32, (y * 4) as f32, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

fn mouse(
    mut commands: Commands,
    windows: Res<Windows>,
    treeaccess: Res<NNTree>,
    mut query: Query<&mut Transform, With<Cursor>>,
    ms_buttons: Res<Input<MouseButton>>,
) {
    let use_mouse = ms_buttons.pressed(MouseButton::Left);
    let win = windows.get_primary().unwrap();
    if let Some(mut pos) = win.cursor_position() {
        pos.x -= win.width() / 2.0;
        pos.y -= win.height() / 2.0;
        let mut transform = query.single_mut();
        if let Some((_pos, entity)) = treeaccess.nearest_neighbour(pos.extend(0.0)) {
            if use_mouse {
                transform.translation = pos.extend(1.0); // I don't really know what this is here for
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn color(
    windows: Res<Windows>,
    treeaccess: Res<NNTree>,
    mut query: Query<&mut Sprite, With<NearestNeighbourComponent>>,
) {
    let win = windows.get_primary().unwrap();
    if let Some(mut pos) = win.cursor_position() {
        pos.x -= win.width() / 2.0;
        pos.y -= win.height() / 2.0;

        for (_, entity) in treeaccess.within_distance(pos.extend(0.0), 50.0) {
            if let Ok(mut sprite) = query.get_mut(entity) {
                sprite.color = Color::BLACK;
            }
        }
    }
}

fn reset_color(mut query: Query<&mut Sprite, With<NearestNeighbourComponent>>) {
    for mut sprite in &mut query {
        sprite.color = Color::ORANGE_RED;
    }
}

fn movement(mut query: Query<&mut Transform, With<NearestNeighbourComponent>>) {
    for mut pos in &mut query {
        let goal = pos.translation - Vec3::ZERO;
        pos.translation += goal.normalize_or_zero();
    }
}

fn collide_wall(
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<NearestNeighbourComponent>>,
) {
    let win = windows.get_primary().unwrap();

    let w = win.width() / 2.0;
    let h = win.height() / 2.0;

    for mut pos in &mut query {
        let [x, y] = pos.translation.xy().to_array();
        if y < -h || x < -w || y > h || x > w {
            pos.translation = pos.translation.normalize_or_zero();
        }
    }
}
