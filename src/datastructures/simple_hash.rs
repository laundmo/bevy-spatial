use rustc_hash::FxHashMap;
use smallvec::{smallvec, SmallVec};
type Vec<T> = SmallVec<[T; 5]>;

pub struct SpatialHash2d<D> {
    map: FxHashMap<glam::IVec2, Vec<(glam::Vec2, D)>>,
    precision: glam::Vec2,
}

impl<D> SpatialHash2d<D> {
    pub fn new(precision: i32) -> Self {
        Self {
            map: FxHashMap::default(),
            precision: glam::Vec2::splat(precision as f32),
        }
    }

    #[inline(always)]
    fn to_key(&self, pos: glam::Vec2) -> glam::IVec2 {
        (pos * self.precision).as_ivec2()
    }

    pub fn insert(&mut self, pos: glam::Vec2, data: D) {
        let key = self.to_key(pos);
        match self.map.get_mut(&key) {
            Some(vec) => vec.push((pos, data)),
            None => {
                self.map.insert(key, smallvec![(pos, data)]);
            }
        };
    }

    pub fn remove(&mut self, pos: glam::Vec2) -> Option<(glam::Vec2, D)> {
        let key = self.to_key(pos);
        let vec = self.map.get_mut(&key)?;
        let pos = vec.iter().position(|i| i.0 == pos)?;
        let data = vec.swap_remove(pos);
        if vec.is_empty() {
            self.map.remove(&key);
        }
        Some(data)
    }

    pub fn update(&mut self, old_pos: glam::Vec2, new_pos: glam::Vec2) -> bool {
        let old_key = self.to_key(old_pos);
        let new_key = self.to_key(new_pos);

        // no need to do anything
        if old_key == new_key {
            return true;
        };

        if let Some(data) = self.remove(old_pos) {
            self.insert(data.0, data.1);
            true
        } else {
            false
        }
    }

    pub fn get(&self, pos: glam::Vec2) -> Option<&D> {
        self.map
            .get(&self.to_key(pos))
            .and_then(|v| v.iter().find(|i| i.0 == pos).map(|i| &i.1))
    }

    pub fn get_in_cell(&self, pos: glam::Vec2) -> Option<&[(glam::Vec2, D)]> {
        self.map.get(&self.to_key(pos)).map(Vec::as_slice)
    }

    pub fn get_in_box(&self, center: glam::Vec2, radius: f32) {
        let radius_limit = (radius * self.precision.x) as i32;
        let spiral = crate::spiral::Spiral::new(self.to_key(center), radius_limit);
        let mut results = Vec::new();
        for pos in spiral {
            if let Some(v) = self.map.get(&pos) {
                results.push(v); // TODO: slice
            }
        }
    }
}
