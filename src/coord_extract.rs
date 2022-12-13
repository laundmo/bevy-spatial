use std::marker::PhantomData;

use bevy::prelude::*;

// Trait bound aliases
trait TComp: Component + Send + Sync + 'static {}
impl<T> TComp for T where T: Component + Send + Sync + 'static {}

// Trait bound aliases
trait TUnit: PartialEq + Send + Sync + 'static {}
impl<T> TUnit for T where T: PartialEq + Send + Sync + 'static {}

#[derive(Copy, Clone, Debug)]
pub struct EntityPoint<Unit: TUnit> {
    pub vec: Unit,
    pub entity: Entity,
}

pub type EntityPoint2D = EntityPoint<Vec2>;

pub type EntityPoint3D = EntityPoint<Vec3>;

impl EntityPoint2D {
    pub fn new(vec: Vec2, entity: Entity) -> Self {
        Self { vec, entity }
    }
}

impl EntityPoint3D {
    pub fn new(vec: Vec3, entity: Entity) -> Self {
        Self { vec, entity }
    }
}

#[derive(Resource)]
struct EntityCoords<Comp: TComp, T> {
    component: PhantomData<Comp>,
    coords: Vec<T>,
}

impl<Comp: TComp, T> Default for EntityCoords<Comp, T> {
    fn default() -> Self {
        Self {
            component: PhantomData,
            coords: Vec::new(),
        }
    }
}
pub type EntityCoords2D<TComp> = EntityCoords<TComp, EntityPoint2D>;

pub type EntityCoords3D<TComp> = EntityCoords<TComp, EntityPoint3D>;

pub struct Coordinates<Comp, T> {
    component: PhantomData<Comp>,
    point: PhantomData<T>,
    delay: Timer,
}

pub type Coordinates2D<Comp> = Coordinates<Comp, EntityPoint2D>;
pub type Coordinates3D<Comp> = Coordinates<Comp, EntityPoint3D>;

impl<Comp: TComp> Plugin for Coordinates2D<Comp> {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityCoords2D<Comp>>()
            .add_system_set
            .add_system(add_all_2D::<Comp>);
    }
}

pub fn add_all_2D<Comp: TComp>(
    mut coords: ResMut<EntityCoords2D<Comp>>,
    commands: Commands,
    all: Query<(Entity, &Transform), With<Comp>>,
) {
    coords.coords = all
        .iter()
        .map(|(e, t)| EntityPoint2D::new(t.translation.truncate(), e))
        .collect();
}
