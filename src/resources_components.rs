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

/// Internal resource used for fixed timestep without repeats.
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
