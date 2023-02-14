use std::{collections::BTreeMap, marker::PhantomData};

use bevy::{ecs::storage::SparseSet, prelude::*};

use crate::{point::SpatialPoint, spatial_access::SpatialAccess};

// Trait bound aliases
pub trait TComp: Component + Send + Sync + 'static {}
impl<T> TComp for T where T: Component + Send + Sync + 'static {}

#[derive(Resource, Debug)]
pub struct SpatialData<T: SpatialPoint + 'static, Comp: TComp> {
    all: SparseSet<Entity, T>,
    changed: Vec<Entity>,
    removed: Vec<Entity>,
    rebuild_full: bool,
    pd: PhantomData<Comp>,
}

impl<T, Comp> SpatialData<T, Comp>
where
    T: SpatialPoint,
    Comp: TComp,
{
    pub fn new() -> Self {
        SpatialData {
            all: SparseSet::new(),
            changed: Vec::new(),
            removed: Vec::new(),
            // data,
            rebuild_full: false,
            pd: PhantomData,
        }
    }

    // Added as well as Moved
    pub fn add_changed(&mut self, entity: Entity, point: T) {
        let p = self.all.get_or_insert_with(entity, || {
            self.changed.push(entity);
            point
        });

        if p != &point {
            self.changed.push(entity);
            *p = point;
        }
    }

    /// Set the entire SpatialData to a iterators items.
    ///
    /// Note: This can be slower than `add_changed` if a majority of the coordinates are the same.
    pub fn set_all(&mut self, all: impl ExactSizeIterator<Item = (Entity, T)>) {
        self.changed.clear();
        self.removed.clear();
        self.all = SparseSet::with_capacity(all.len());
        for (entity, point) in all {
            self.all.insert(entity, point);
        }
        self.rebuild_full = true;
    }

    pub fn remove(&mut self, entity: Entity) {
        if self.all.remove(entity).is_some() {
            self.removed.push(entity);
        };
    }
}
