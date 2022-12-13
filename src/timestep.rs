use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{Component, Res, ResMut, Resource},
    time::{Time, Timer},
};

/// Resource used for fixed timestep without repeats in the same frame (builtin timestep may run the system multiple times per frame).
///
/// To modify the timestep at runtime, a system like this can be used:
/// ```rust
/// fn update_timestep(
///     mut step: ResMut<TimestepElapsed<NearestNeighbourMarker>>,
/// ) {
///     if some_condition {
///         step.set_duration(Duration::from_millis(15)); // only update spatial datastructure every 15ms.
///     }
/// }
/// ```
/// `NearestNeighbourMarker` in this case refers to the (marker) component you also passed to the Plugin.
#[derive(Resource, Default)]
pub struct TimestepElapsed<TComp>(pub Timer, pub(crate) PhantomData<TComp>);

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

pub fn run_if_elapsed<TComp>(
    mut elapsed: ResMut<TimestepElapsed<TComp>>,
    time: Res<Time>,
) -> ShouldRun
where
    TComp: Component,
{
    if elapsed.tick(time.delta()).finished() {
        elapsed.reset();
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
