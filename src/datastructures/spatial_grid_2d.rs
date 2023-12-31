use crate::utils::{Grid2D, IRect, Rect};
use itertools::Itertools;

use std::{fmt::Debug, ops::Div, time::Instant};

#[cfg(feature = "trace")]
use tracing::{span, Level};

#[derive(Debug, Clone)]
pub struct CoordMapping {
    src: Rect,
    dest: glam::UVec2,
    scale: glam::Vec2,
    translation: glam::Vec2,
}

impl CoordMapping {
    pub fn new(from: Rect, to: glam::UVec2) -> Self {
        let scale = to.as_vec2() / (from.max() - from.min());
        let translation = from.min() * scale;
        Self {
            src: from,
            dest: to,
            scale,
            translation,
        }
    }

    #[inline]
    pub fn map_point(&self, point: glam::Vec2) -> glam::Vec2 {
        ((point * self.scale) + self.translation).floor()
    }

    #[inline]
    pub fn src(&self) -> &Rect {
        &self.src
    }

    #[inline]
    pub fn dest(&self) -> &glam::UVec2 {
        &self.dest
    }
}

#[derive(Debug)]
pub struct FixedSizeGrid<D> {
    rect: CoordMapping,
    cell_size: f32,
    data: Vec<(usize, glam::Vec2, D)>,
    grid: Grid2D<usize>,
    // the following variables are taken out of Grid2D and CoordMapping for making sure the hot path can be optimised as much as possible
    width: u32,
    scale: glam::IVec2,
    translation: glam::IVec2,
}

// TODO: Removed Debug constraint
impl<D: Debug> FixedSizeGrid<D> {
    pub fn new(anchor: glam::Vec2, cell_size: f32, grid_size: glam::UVec2) -> Self {
        let cm = CoordMapping::new(
            Rect::from_center_size(anchor, grid_size.as_vec2() * cell_size),
            grid_size,
        );
        FixedSizeGrid {
            rect: cm.clone(),
            cell_size,
            data: Vec::new(),
            grid: Grid2D::new(grid_size, usize::MAX),
            width: grid_size.x,
            scale: cm.scale.floor().as_ivec2(),
            translation: cm.translation.floor().as_ivec2(),
        }
    }

    pub fn contains_cell(&self, cell: glam::UVec2) -> bool {
        cell.cmplt(*self.rect.dest()).all()
    }

    #[inline]
    pub fn size(&self) -> &glam::UVec2 {
        self.rect.dest()
    }

    #[inline]
    pub fn src_rect(&self) -> &Rect {
        self.rect.src()
    }

    #[inline]
    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    #[inline]
    pub fn get_mapped_point(&self, point: glam::Vec2) -> glam::Vec2 {
        self.rect.map_point(point)
    }

    #[inline]
    pub fn get_index_clamped(&self, point: glam::Vec2) -> usize {
        self.grid
            .get_index(self.rect.map_point(point).as_uvec2().min(*self.size() - 1))
    }

    #[inline]
    fn get_index_clamped_inlined(&self, point: glam::Vec2) -> usize {
        let index = ((point.as_ivec2() * self.scale) + self.translation)
            .as_uvec2()
            .min(*self.size() - 1);
        (index.y * self.width + index.x) as usize
    }

    pub fn update(&mut self, src: impl ExactSizeIterator<Item = (glam::Vec2, D)>) {
        // let s = Instant::now();

        self.data = src
            .map(|(v, d)| (self.get_index_clamped_inlined(v), v, d))
            .collect();
        // self.data = Vec::with_capacity(src.len());
        // for (v, d) in src {
        //     self.data.push((self.get_index_clamped_inlined(v), v, d));
        // }

        // println!("coll={}", s.elapsed().as_nanos());
        // let s = Instant::now();

        // faster sort\
        self.data.sort_unstable_by_key(|e| e.0);
        // println!("sort={}", s.elapsed().as_nanos());
        // let s = Instant::now();
        self.data
            .iter()
            .map(|(k, _, _)| *k)
            .enumerate()
            .dedup_by(|a, b| a.1 == b.1)
            .for_each(|(start_idx, k)| {
                self.grid.write_direct(k, start_idx);
            });
        // println!("grid={}", s.elapsed().as_nanos());
    }

    pub fn get_cell_data(&self, point: glam::Vec2) -> Option<impl Iterator<Item = &'_ D>> {
        let index = self.get_index_clamped(point);
        let start_idx = *self.grid.get_direct(index);
        if start_idx == usize::MAX {
            return None;
        }

        Some(self.data[start_idx..].iter().filter_map(
            move |(k, _, d)| {
                if *k == index {
                    Some(d)
                } else {
                    None
                }
            },
        ))
    }

    pub fn get_in_radius(&self, center: glam::Vec2, radius: f32) -> InRadiusIter<D> {
        InRadiusIter::new(self, center, radius)
    }
}
#[inline(never)]
pub fn asm_test(g: FixedSizeGrid<usize>, v: glam::Vec2) {
    g.get_mapped_point(v);
}

// TODO: Optimisation idea: scan row-wise or column-wise (whichever is major) to scan entire runs at once. should allow better cache usage
// data should also already be sorted in the same major because the sort key is the 2d grid index
pub struct InRadiusIter<'g, D> {
    grid: &'g FixedSizeGrid<D>,
    radius_squared: f32,
    spiral: crate::spiral::Spiral,
    center: glam::Vec2,
    current_inner: Option<(usize, std::slice::Iter<'g, (usize, glam::Vec2, D)>)>,
}

impl<'g, D: Debug> InRadiusIter<'g, D> {
    fn new(grid: &'g FixedSizeGrid<D>, center: glam::Vec2, radius: f32) -> Self {
        let radius_limit = radius.ceil().div(grid.cell_size).ceil() as i32;
        Self {
            grid,
            radius_squared: radius.powi(2),
            spiral: crate::spiral::Spiral::new(
                grid.get_mapped_point(center).as_ivec2(),
                radius_limit,
            ),
            center,
            current_inner: None,
        }
    }
}

impl<'g, D: Debug> Iterator for InRadiusIter<'g, D> {
    type Item = &'g D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_inner.is_none() {
            while self.current_inner.is_none() {
                let cell = self.spiral.next()?.max(glam::IVec2::ZERO).as_uvec2(); // load-bearing try. if this is never None, the iterator never stops.
                if !self.grid.contains_cell(cell) {
                    continue;
                }
                let querying_key = self.grid.grid.get_index(cell);

                let i = self.grid.grid.get_direct(querying_key);
                if *i != usize::MAX {
                    let slice = &self.grid.data[*i..];
                    self.current_inner = Some((querying_key, slice.iter()));
                };
            }
        }
        if let Some(ref mut inner) = self.current_inner {
            let (querying_key, ref mut sliceiter) = inner;
            match sliceiter.next() {
                Some((k, v, d)) => {
                    if *k != *querying_key {
                        self.current_inner = None;
                        return self.next();
                    }
                    if v.distance_squared(self.center) < self.radius_squared {
                        Some(d)
                    } else {
                        self.next()
                    }
                }
                None => {
                    self.current_inner = None;
                    self.next()
                }
            }
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use divan::black_box;
    use rand::Rng;

    use super::FixedSizeGrid;

    fn gen_data(n: usize, mult: f32) -> Vec<(glam::Vec2, usize)> {
        let mut data = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let x: f32 = rng.gen();
            let y: f32 = rng.gen();

            data.push((glam::vec2(x * mult, y * mult), rng.gen::<usize>()));
        }
        data
    }

    #[cfg(feature = "trace")]
    #[test]
    fn test_many_insert() {
        use std::{thread, time::Duration};

        use tracing::*;
        use tracing_subscriber::layer::SubscriberExt;

        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()),
        )
        .expect("set up the subscriber");
        thread::sleep(Duration::from_secs(20));
        let _guard = span!(Level::INFO, "test").entered();
        let mut g = FixedSizeGrid::new(glam::vec2(100., 100.), 10., glam::uvec2(10, 10));

        g.update(black_box(gen_data(5000000, 100.)).into_iter());
    }
}
//     fn test_cell() {
//         // ----- x
//         // |0|1|
//         // -----
//         // |2|3|
//         // -----
//         // y
//         let g: FixedSizeGrid<()> = FixedSizeGrid::new(glam::vec2(1., 1.), 1., glam::uvec2(2, 2));
//         let cells = [
//             glam::vec2(0.1, 0.1),
//             glam::vec2(1.1, 0.),
//             glam::vec2(0., 1.1),
//             glam::vec2(1.1, 1.1),
//         ];
//         for (i, c) in cells.iter().enumerate() {
//             assert_eq!(g.get_mapped_point(*c), c.floor().as_ivec2());
//         }
//     }

//     #[test]
//     fn test_cell_oob() {
//         // ----- x
//         // |0|1|
//         // -----
//         // |2|3|
//         // -----
//         // y
//         let g: FixedSizeGrid<()> = FixedSizeGrid::new(glam::vec2(1., 1.), 1., glam::uvec2(2, 2));
//         let cells = [
//             (glam::vec2(-1., -1.), glam::uvec2(0, 0)),
//             (glam::vec2(10., 0.), glam::uvec2(1, 0)),
//             (glam::vec2(0., 10.), glam::uvec2(0, 1)),
//             (glam::vec2(10., 10.), glam::uvec2(1, 1)),
//         ];
//         for (i, o) in cells.iter() {
//             assert_eq!(g.get_key(*i), Some(*o));
//         }
//         let oob_cells = [
//             glam::ivec2(-100, -100),
//             glam::ivec2(100, 0),
//             glam::ivec2(0, 100),
//             glam::ivec2(100, 100),
//         ];
//         for c in oob_cells.iter() {
//             assert_eq!(g.get_cell(*c, super::OOBStrategy::Ignore), None);
//         }
//     }

//     #[test]
//     fn test() {
//         let mut g: FixedSizeGrid<usize> =
//             FixedSizeGrid::new(glam::vec2(50., 50.), 10., glam::uvec2(10, 10));

//         g.update(
//             vec![
//                 (glam::vec2(0.0, 0.0), 0),
//                 (glam::vec2(1.0, 0.0), 1),
//                 (glam::vec2(0.0, 1.0), 2),
//                 (glam::vec2(15.0, 0.0), 3),
//                 (glam::vec2(0.0, 15.0), 4),
//                 (glam::vec2(15.0, 15.0), 5),
//                 (glam::vec2(20.0, 0.0), 6),
//             ]
//             .into_iter(),
//         );
//         assert_eq!(
//             g.get_in_radius(glam::vec2(1.0, 1.0), 5.).collect_vec(),
//             vec![&0, &1, &2]
//         );
//     }

//     #[test]
//     fn test_many_cells() {
//         let mut g: FixedSizeGrid<usize> =
//             FixedSizeGrid::new(glam::vec2(25., 15.), 10., glam::uvec2(5, 3));
//         let mut dat = vec![(glam::vec2(1., 1.), 999); 20];
//         let mut c = 0;
//         for y in 0..3 {
//             for x in 0..5 {
//                 dat.push((glam::ivec2(x * 10, y * 10).as_vec2(), c));
//                 c += 1;
//             }
//         }
//         dbg!(&dat);
//         g.update(dat.clone().into_iter());

//         g.update(dat.clone().into_iter());

//         g.update(dat.clone().into_iter());

//         let point = glam::vec2(10., 0.);
//         let key = g.get_key(point).unwrap();
//         dbg!(&g, &g.data[3], g.grid.get_index(key), g.grid[key]);
//         g.get_cell_data(point).unwrap().for_each(|i| {
//             dbg!(i);
//         });
//     }
// }
