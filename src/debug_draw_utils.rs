use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::{RectAABB, AABB};

pub fn line_rect(aabb: RectAABB, lines: &mut DebugLines) {
    let lower = aabb.lower().extend(0.0);
    let upper = aabb.upper().extend(0.0);
    lines.line_colored(lower, Vec3::new(lower.x, upper.y, 0.0), 0.0, Color::GREEN);
    lines.line_colored(lower, Vec3::new(upper.x, lower.y, 0.0), 0.0, Color::GREEN);
    lines.line_colored(upper, Vec3::new(lower.x, upper.y, 0.0), 0.0, Color::GREEN);
    lines.line_colored(upper, Vec3::new(upper.x, lower.y, 0.0), 0.0, Color::GREEN);
}
