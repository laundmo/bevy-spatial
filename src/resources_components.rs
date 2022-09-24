use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use bevy::prelude::*;

/// Internal component which tracks the last position at which the entity was updated in the tree.
#[derive(Component)]
pub struct MovementTracked<T> {
    pub lastpos: Vec3,
    pub component_type: PhantomData<T>,
}

impl<T> MovementTracked<T> {
    pub fn new(last: Vec3) -> Self {
        MovementTracked {
            lastpos: last,
            component_type: PhantomData,
        }
    }
}

/// Resource used for fixed timestep without repeats in the same frame (builtin timestep may run the system multiple times per frame).
///
/// To modify the timestep at runtime, a system like this can be used:
/// ```rs
/// fn update_timestep(
///     mut step: ResMut<TimestepElapsed<NearestNeighbourMarker>>,
/// ) {

///     if some_condition {
///         step.set_duration(Duration::from_millis(15)); // only update timestep every 15ms.
///     }
/// }
/// ```
/// `NearestNeighbourMarker` in this case refers to the (marker) component you also passed to the Plugin.
#[derive(Default)]
pub struct TimestepElapsed<TComp>(pub Timer, pub PhantomData<TComp>);

impl<TComp> Deref for TimestepElapsed<TComp> {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TComp> DerefMut for TimestepElapsed<TComp> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
