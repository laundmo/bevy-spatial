use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use bevy::{
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

pub fn on_timer_changeable<TRes>(mut elapsed: ResMut<TRes>, time: Res<Time>) -> bool
where
    TRes: Deref<Target = Timer> + DerefMut<Target = Timer> + Resource,
{
    elapsed.tick(time.delta());
    elapsed.just_finished()
}
