use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

use crate::{AABBextras2d, RectAABB, AABB};

pub struct DebugAABB<TComp> {
    pub pd: PhantomData<TComp>,
}

impl<TComp> Default for DebugAABB<TComp> {
    fn default() -> Self {
        Self { pd: default() }
    }
}

impl<TComp> Plugin for DebugAABB<TComp>
where
    TComp: Component + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugLinesPlugin::default())
            .add_system(aabb_show::<TComp>);
    }
}

fn aabb_show<TComp>(
    mut lines: ResMut<DebugLines>,
    mut query: Query<(Entity, &Transform), With<TComp>>,
) where
    TComp: Component + Send + Sync + 'static,
{
    for [item1, item2] in query.iter_combinations::<2>() {
        let bb1: RectAABB = item1.into();
        let bb2: RectAABB = item2.into();
        bb1.debug_draw_line(&mut lines, Color::GREEN);
        bb2.debug_draw_line(&mut lines, Color::GREEN);
        if let Some(overlap) = bb1.overlap(&bb2) {
            overlap.debug_draw_line(&mut lines, Color::RED);
        }
    }
}
