use std::{marker::PhantomData, time::Duration};

use bevy::{
    prelude::{Local, Res, Resource},
    time::{Time, Timer, TimerMode},
};

use crate::TComp;

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
pub struct TimestepLength<Comp>(pub Duration, pub(crate) PhantomData<Comp>);

impl<Comp> TimestepLength<Comp> {
    /// Set the length of the timestep.
    pub fn set_duration(&mut self, duration: Duration) {
        self.0 = duration;
    }

    /// Get the length of the timestep.
    pub fn get_duration(&self) -> Duration {
        self.0
    }
}

pub fn on_timer_changeable<Comp>(
    length: Res<TimestepLength<Comp>>,
    time: Res<Time>,
    mut timer: Local<Timer>,
) -> bool
where
    Comp: TComp,
{
    if length.get_duration() != timer.duration() {
        timer.set_mode(TimerMode::Repeating);
        timer.set_duration(length.get_duration());
    }
    timer.tick(time.delta());
    timer.just_finished()
}
