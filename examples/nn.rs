use bevy::prelude::*;
use bevy_spatial::{DefaultParams, RTreeAccess2D, RTreePlugin2D};

#[derive(Component)]
struct NearestNeighbour;

#[derive(Component)]
struct MoveTowards;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RTreePlugin2D::<NearestNeighbour, DefaultParams> { ..default() })
        .add_startup_system(setup)
        .add_system(mouseclick)
        .add_system(move_to)
        .run();
}

type NNTree = RTreeAccess2D<NearestNeighbour, DefaultParams>;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for x in -6..6 {
        for y in -6..6 {
            commands
                .spawn()
                .insert(NearestNeighbour)
                .insert_bundle(SpriteBundle {
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
                });
        }
    }
}

fn mouseclick(mut commands: Commands, mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>) {
    let win = windows.get_primary().unwrap();
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(mut pos) = win.cursor_position() {
            pos.x -= win.width() / 2.0;
            pos.y -= win.height() / 2.0;
            commands
                .spawn()
                .insert(MoveTowards)
                .insert_bundle(SpriteBundle {
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
                });
        }
    }
}

fn move_to(
    treeaccess: Res<NNTree>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MoveTowards>>,
) {
    for mut transform in query.iter_mut() {
        let pos = transform.translation.truncate();
        if let Some(nearest) = treeaccess.tree.nearest_neighbor(&pos.to_array()) {
            let towards = nearest.pos - pos;
            transform.translation +=
                (towards.normalize() * time.delta_seconds() * 64.0).extend(0.0);
        }
    }
}
