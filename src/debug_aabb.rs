use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

use crate::{debug_draw_utils::line_rect, RectAABB};

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
    for item in query.iter() {
        let bb: RectAABB = item.into();
        line_rect(bb, &mut lines);
    }
}
