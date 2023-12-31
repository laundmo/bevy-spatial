use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_screen_diagnostics::ScreenDiagnosticsPlugin::default())
        .add_plugins(bevy_screen_diagnostics::ScreenFrameDiagnosticsPlugin)
        .add_plugins(bevy_screen_diagnostics::ScreenEntityDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (populate, update, (reset_color, color_grid).chain()),
        )
        .init_resource::<Grid>()
        .run();
}

#[derive(Resource, DerefMut, Deref)]
struct Grid(simple_spatial_hash::FixedSizeGrid<Entity>);

impl Default for Grid {
    fn default() -> Self {
        Self(simple_spatial_hash::FixedSizeGrid::new(
            Vec2::new(100., 100.),
            20.,
            UVec2::new(10, 10),
        ))
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Point;

fn populate(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    rng.gen::<f32>() * 200.,
                    rng.gen::<f32>() * 200.,
                    rng.gen::<f32>() * 200.,
                )),
                ..default()
            },
            Point,
        ));
    }
}

fn update(mut g: ResMut<Grid>, query: Query<(&Transform, Entity), With<Point>>) {
    let mut vec = Vec::with_capacity(query.iter().len());
    for (t, e) in &query {
        vec.push((t.translation.xy(), e));
    }
    g.update(vec.into_iter());
}

fn reset_color(mut query: Query<&mut Sprite, With<Point>>) {
    for mut c in &mut query {
        c.color += Color::rgb(0.01, 0.01, 0.01);
    }
}

fn color_grid(
    l: Res<Grid>,
    mut query: Query<&mut Sprite, With<Point>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    mut gizmos: Gizmos,
) {
    let src = l.src_rect().min();
    let s = l.size();
    let cell_size = l.cell_size();
    for row in 0..(s.y + 1) {
        let y = row as f32 * cell_size;
        gizmos.line_2d(
            src + Vec2::new(src.x, y),
            src + Vec2::new(src.x + cell_size * s.y as f32, y),
            Color::RED,
        )
    }
    for col in 0..(s.x + 1) {
        let x = col as f32 * cell_size;
        gizmos.line_2d(
            src + Vec2::new(x, src.y),
            src + Vec2::new(x, src.y + cell_size * s.x as f32),
            Color::RED,
        )
    }

    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    let on_grid = l.get_mapped_point(point);
    gizmos.circle_2d(
        src + (on_grid * cell_size) + cell_size / 2.,
        10.,
        Color::BLACK,
    );

    for e in l.get_in_radius(point, 15.) {
        if let Ok(mut c) = query.get_mut(*e) {
            c.color = Color::BLACK;
        }
    }
}
