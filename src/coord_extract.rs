use std::marker::PhantomData;

use bevy::prelude::*;

use crate::types::EntityPoint;

// Trait bound aliases
trait TComp: Component + Send + Sync + 'static {}
impl<T> TComp for T where T: Component + Send + Sync + 'static {}

#[derive(Resource)]
struct EntityCoords<Comp: TComp, EP: EntityPoint> {
    component: PhantomData<Comp>,
    coords: Vec<EP>,
}

impl<Comp: TComp, EP: EntityPoint> Default for EntityCoords<Comp, EP> {
    fn default() -> Self {
        Self {
            component: PhantomData,
            coords: Vec::new(),
        }
    }
}

pub struct Coordinates<Comp, Point> {
    component: PhantomData<Comp>,
    point: PhantomData<Point>,
    delay: Timer,
}

impl<Comp: TComp, EP: EntityPoint> Plugin for Coordinates<Comp, EP> {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityCoords<Comp, EP>>()
            .add_system_set
            .add_system(add_all_2D::<Comp>);
    }
}

pub fn add_all_2D<Comp: TComp, EP: EntityPoint>(
    mut coords: ResMut<EntityCoords<Comp, EP>>,
    commands: Commands,
    all: Query<(Entity, &Transform), With<Comp>>,
) {
    coords.coords = all.iter().map(|(e, t)| EP::from_vec3(t, e)).collect();
}
