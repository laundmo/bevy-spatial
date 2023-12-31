use std::fmt::Debug;

#[derive(Debug)]
pub struct IRect {
    min: glam::IVec2,
    max: glam::IVec2,
}

impl IRect {
    #[inline]
    pub fn from_corners(p0: glam::IVec2, p1: glam::IVec2) -> Self {
        Self {
            min: p0.min(p1),
            max: p0.max(p1),
        }
    }

    #[inline]
    pub fn from_center_size(origin: glam::IVec2, size: glam::UVec2) -> Self {
        let half_size = size / 2;
        Self::from_center_half_size(origin, half_size)
    }

    #[inline]
    pub fn from_center_half_size(origin: glam::IVec2, half_size: glam::UVec2) -> Self {
        let half_size = half_size.as_ivec2();
        Self {
            min: origin - half_size,
            max: origin + half_size,
        }
    }
    #[inline]
    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn size(&self) -> glam::IVec2 {
        self.max - self.min
    }

    #[inline]
    pub fn contains(&self, point: glam::IVec2) -> bool {
        (point.cmpge(self.min) & point.cmplt(self.max)).all()
    }

    #[inline]
    pub fn min(&self) -> glam::IVec2 {
        self.min
    }

    #[inline]
    pub fn max(&self) -> glam::IVec2 {
        self.max
    }
}

pub trait IntoIRect {
    fn into_rect(self) -> IRect;
}
impl IntoIRect for IRect {
    fn into_rect(self) -> IRect {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    min: glam::Vec2,
    max: glam::Vec2,
}

impl Rect {
    #[inline]
    pub fn from_corners(p0: glam::Vec2, p1: glam::Vec2) -> Self {
        Self {
            min: p0.min(p1),
            max: p0.max(p1),
        }
    }

    #[inline]
    pub fn from_center_size(origin: glam::Vec2, size: glam::Vec2) -> Self {
        let half_size = size / 2.;
        Self::from_center_half_size(origin, half_size)
    }

    #[inline]
    pub fn from_center_half_size(origin: glam::Vec2, half_size: glam::Vec2) -> Self {
        Self {
            min: origin - half_size,
            max: origin + half_size,
        }
    }
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn size(&self) -> glam::Vec2 {
        self.max - self.min
    }

    #[inline]
    pub fn contains(&self, point: glam::Vec2) -> bool {
        (point.cmpge(self.min) & point.cmple(self.max)).all()
    }
    #[inline]
    pub fn min(&self) -> glam::Vec2 {
        self.min
    }

    #[inline]
    pub fn max(&self) -> glam::Vec2 {
        self.max
    }
}

pub trait IntoRect {
    fn into_rect(self) -> Rect;
}
impl IntoRect for Rect {
    fn into_rect(self) -> Rect {
        self
    }
}

//TODO: make data private
/// Row-Major 2D Grid
pub struct Grid2D<T> {
    size: glam::UVec2,
    pub data: Vec<T>,
}

impl<T: Clone> Grid2D<T> {
    pub fn new(size: glam::UVec2, default: T) -> Self {
        let data = vec![default; (size.x * size.y) as usize];
        Self { size, data }
    }
}

impl<T> Grid2D<T> {
    #[inline]
    pub fn size(&self) -> glam::UVec2 {
        self.size
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.size.x
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.size.y
    }

    #[inline(always)]
    pub fn get_index(&self, index: glam::UVec2) -> usize {
        (index.y * self.width() + index.x) as usize
    }

    #[inline(always)]
    pub fn write_direct(&mut self, i: usize, data: T) {
        self.data[i] = data;
    }

    #[inline(always)]
    pub fn get_direct(&self, i: usize) -> &T {
        &self.data[i]
    }

    #[inline]
    pub fn get(&self, index: glam::UVec2) -> Option<&T> {
        let i = self.get_index(index);
        self.data.get(i)
    }
}

impl<T> std::ops::Index<glam::UVec2> for Grid2D<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: glam::UVec2) -> &Self::Output {
        &self.data[self.get_index(index)]
    }
}

impl<T> std::ops::IndexMut<glam::UVec2> for Grid2D<T> {
    fn index_mut(&mut self, index: glam::UVec2) -> &mut Self::Output {
        let idx = self.get_index(index);
        &mut self.data[idx]
    }
}

impl<T: Debug> Debug for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        let width = f.width().unwrap_or(
            /*
                Conditionally calculate the longest item by default.
            */
            self.data
                .iter()
                .map(|i| format!("{i:?}").len())
                .max()
                .unwrap(),
        );
        writeln!(f, "[")?;
        for row in 0..self.height() as usize {
            write!(f, "    [")?;
            for col in 0..self.width() as usize {
                let cell = glam::uvec2(col as u32, row as u32);
                let i = &self.data[self.get_index(cell)];
                write!(f, " {i:width$.precision$?}",)?;
            }
            writeln!(f, "],")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Grid2D;

    #[test]
    fn test_grid() {
        let mut grid = Grid2D::new(glam::uvec2(10, 10), 0);
        let i = glam::uvec2(2, 2);
        grid[i] = 5;
        assert_eq!(grid.data[22], 5);
    }
}
