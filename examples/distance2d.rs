use std::ops::Deref;
use std::time::Duration;

use bevy::{
    color::palettes::css as csscolors,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Vec3Swizzles,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_spatial::{AutomaticUpdate, SpatialStructure};
use bevy_spatial::{SpatialAccess, kdtree::KDTree2};
// marker for entities tracked by the KDTree
#[derive(Component, Default)]
struct NearestNeighbourComponent;

// marker for the "cursor" entity
#[derive(Component)]
struct Cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // Add the plugin, which takes the tracked component as a generic.
        .add_plugins(
            AutomaticUpdate::<NearestNeighbourComponent>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(1)),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(Mouse2D { pos: Vec2::ZERO })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_mouse_pos,
                (
                    mouse,
                    color,
                    color_rect,
                    reset_color.before(color),
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
    commands.spawn(Camera2d);
    commands.spawn((
        Cursor,
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        Transform {
            translation: Vec3::ZERO,
            ..default()
        },
    ));
    let sprite = Sprite {
        color: csscolors::ORANGE_RED.into(),
        custom_size: Some(Vec2::new(6.0, 6.0)),
        ..default()
    };
    for x in -100..100 {
        for y in -100..100 {
            commands.spawn((
                NearestNeighbourComponent,
                sprite.clone(),
                Transform {
                    translation: Vec3::new((x * 4) as f32, (y * 4) as f32, 0.0),
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
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut mouse: ResMut<Mouse2D>,
) {
    let (cam, cam_t) = camera.deref();
    if let Some(w_pos) = window.cursor_position() {
        if let Ok(pos) = cam.viewport_to_world_2d(cam_t, w_pos) {
            mouse.pos = pos;
        }
    }
}

fn mouse(
    mut commands: Commands,
    mouse: Res<Mouse2D>,
    treeaccess: Res<NNTree>,
    mut transform: Single<&mut Transform, With<Cursor>>,
    ms_buttons: Res<ButtonInput<MouseButton>>,
) {
    let use_mouse = ms_buttons.pressed(MouseButton::Left);

    if let Some((_pos, entity)) = treeaccess.nearest_neighbour(mouse.pos) {
        transform.translation = mouse.pos.extend(0.0); // I don't really know what this is here for

        if use_mouse {
            commands.entity(entity.unwrap()).despawn();
        }
    }
}

fn color(
    treeaccess: Res<NNTree>,
    mouse: Res<Mouse2D>,
    mut query: Query<&mut Sprite, With<NearestNeighbourComponent>>,
) {
    for (_, entity) in treeaccess.within_distance(mouse.pos, 50.0) {
        if let Ok(mut sprite) = query.get_mut(entity.unwrap()) {
            sprite.color = Color::BLACK;
        }
    }
}

fn color_rect(
    treeaccess: Res<NNTree>,
    mouse: Res<Mouse2D>,
    mut query: Query<&mut Sprite, With<NearestNeighbourComponent>>,
) {
    let mut p1 = mouse.pos;
    let mut p2 = Vec2::from([100.0, -100.0]);

    if p1.x > p2.x {
        (p1.x, p2.x) = (p2.x, p1.x);
    }

    if p1.y > p2.y {
        (p1.y, p2.y) = (p2.y, p1.y);
    }

    for (_, entity) in treeaccess.within(p1, p2) {
        if let Ok(mut sprite) = query.get_mut(entity.unwrap()) {
            sprite.color = Color::GREEN;
        }
    }
}

fn reset_color(mut query: Query<&mut Sprite, With<NearestNeighbourComponent>>) {
    for mut sprite in &mut query {
        sprite.color = csscolors::ORANGE_RED.into();
    }
}

fn movement(mut query: Query<&mut Transform, With<NearestNeighbourComponent>>) {
    for mut pos in &mut query {
        let goal = pos.translation - Vec3::ZERO;
        pos.translation += goal.normalize_or_zero();
    }
}

fn collide_wall(
    window: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<NearestNeighbourComponent>>,
) {
    let w = window.width() / 2.0;
    let h = window.height() / 2.0;

    for mut pos in &mut query {
        let [x, y] = pos.translation.xy().to_array();
        if y < -h || x < -w || y > h || x > w {
            pos.translation = pos.translation.normalize_or_zero();
        }
    }
}
