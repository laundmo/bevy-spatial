use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Vec3Swizzles,
    prelude::*,
};
use bevy_spatial::{MovingObjectsParams, RTreePlugin2D, SpatialAccess, TreeAccess2D};

#[derive(Component)]
struct NearestNeighbourComponent;

#[derive(Component)]
struct Cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin, which takes the tracked component as a generic.
        .add_plugin(RTreePlugin2D::<NearestNeighbourComponent, MovingObjectsParams> { ..default() })
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
type NNTree = TreeAccess2D<NearestNeighbourComponent, MovingObjectsParams>;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn().insert(Cursor).insert_bundle(SpriteBundle {
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
    });
    let sprite = Sprite {
        color: Color::ORANGE_RED,
        custom_size: Some(Vec2::new(6.0, 6.0)),
        ..default()
    };
    for x in -150..150 {
        for y in -150..150 {
            commands
                .spawn()
                .insert(NearestNeighbourComponent)
                .insert_bundle(SpriteBundle {
                    sprite: sprite.clone(),
                    transform: Transform {
                        translation: Vec3::new((x * 4) as f32, (y * 4) as f32, 0.0),
                        ..default()
                    },
                    ..default()
                });
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
        if let Some(nearest) = treeaccess.nearest_neighbour(pos.extend(0.0)) {
            transform.translation = nearest.0;
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
        pos.x = pos.x - win.width() / 2.0;
        pos.y = pos.y - win.height() / 2.0;

        for (_, entity) in treeaccess.within_distance(pos.extend(0.0), 50.0) {
            let mut sprite = query.get_mut(entity).unwrap();
            sprite.color = Color::BLACK;
        }
    }
}

fn reset_color(mut query: Query<&mut Sprite, With<NearestNeighbourComponent>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::ORANGE_RED;
    }
}

fn movement(mut query: Query<&mut Transform, With<NearestNeighbourComponent>>) {
    for mut pos in query.iter_mut() {
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

    for mut pos in query.iter_mut() {
        let [x, y] = pos.translation.xy().to_array();
        if y < -h || x < -w || y > h || x > w {
            pos.translation = pos.translation.normalize_or_zero();
        }
    }
}
