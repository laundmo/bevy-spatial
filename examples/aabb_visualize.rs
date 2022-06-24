use bevy::prelude::*;
use bevy_spatial::{Rect, AABB};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(mouse)
        .run();
}

#[derive(Component)]
struct Center;

#[derive(Component)]
struct Cursor;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn().insert(Center).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::ORANGE_RED,
            custom_size: Some(Vec2::new(3.0, 3.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(5.0)),
        ..default()
    });

    commands.spawn().insert(Cursor).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(3.0, 3.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(5.0)),
        ..default()
    });
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

fn aabb_draw(mut query: Query<(&mut Transform, Entity)>) {
    let aabbs: Vec<Rect> = vec![];
    for (mut tra, ent) in query.iter_mut() {
        let aabb = Rect::new(&tra, ent);
        aabbs.append(aabb);
    }

    // TODO: draw aabb, maybe only in debug mode?
    // TODO: draw intersections
}
