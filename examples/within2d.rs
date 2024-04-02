use std::time::Duration;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Vec3Swizzles,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_spatial::{kdtree::KDTree2, SpatialAccess, SpatialAABBAccess};
use bevy_spatial::{AutomaticUpdate, SpatialStructure};
// marker for entities tracked by the KDTree
#[derive(Component, Default)]
struct NearestNeighbourComponent;

// marker for the "cursor" entity
#[derive(Component)]
struct Cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin, which takes the tracked component as a generic.
        .add_plugins(
            AutomaticUpdate::<NearestNeighbourComponent>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(1)),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(Mouse2D { pos: Vec2::ZERO })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_mouse_pos,
                (
                    mouse,
                    color_rect,
                    reset_color.before(color_rect),
                    collide_wall,
                    movement,
                ),
            )
                .chain(),
        )
        .run();
}

// type alias for easier usage later
type NNTree = KDTree2<NearestNeighbourComponent>;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
#[derive(Copy, Clone, Resource)]
struct Mouse2D {
    pos: Vec2,
}

fn update_mouse_pos(
    window: Query<&Window, With<PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform)>,
    mut mouse: ResMut<Mouse2D>,
) {
    let win = window.single();
    let (cam, cam_t) = cam.single();
    if let Some(w_pos) = win.cursor_position() {
        if let Some(pos) = cam.viewport_to_world_2d(cam_t, w_pos) {
            mouse.pos = pos;
        }
    }
}

fn mouse(
    mut commands: Commands,
    mouse: Res<Mouse2D>,
    treeaccess: Res<NNTree>,
    mut query: Query<&mut Transform, With<Cursor>>,
    ms_buttons: Res<ButtonInput<MouseButton>>,
) {
    let use_mouse = ms_buttons.pressed(MouseButton::Left);

    let p1 = mouse.pos;
    let p2 = Vec2::from([100.0, -100.0]);

    let mut transform = query.single_mut();

    for (_, entity) in treeaccess.within(p1, p2) {
        if use_mouse {
            commands.entity(entity.unwrap()).despawn();
        }
    }
}

fn color_rect(
    treeaccess: Res<NNTree>,
    mouse: Res<Mouse2D>,
    mut query: Query<&mut Sprite, With<NearestNeighbourComponent>>,
) {
    let p1 = mouse.pos;
    let p2 = Vec2::from([100.0, -100.0]);

    for (_, entity) in treeaccess.within(p1, p2) {
        if let Ok(mut sprite) = query.get_mut(entity.unwrap()) {
            sprite.color = Color::GREEN;
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
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<NearestNeighbourComponent>>,
) {
    let win = window.get_single().unwrap();

    let w = win.width() / 2.0;
    let h = win.height() / 2.0;

    for mut pos in &mut query {
        let [x, y] = pos.translation.xy().to_array();
        if y < -h || x < -w || y > h || x > w {
            pos.translation = pos.translation.normalize_or_zero();
        }
    }
}
