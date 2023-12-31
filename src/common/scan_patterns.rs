#[derive(Debug)]
pub struct Spiral {
    prev_offset: glam::IVec2,
    center: glam::IVec2,
    limit: i32,
}

impl Spiral {
    pub fn new(start: glam::IVec2, limit: i32) -> Self {
        Self {
            center: start,
            limit,
            prev_offset: glam::ivec2(0, 0),
        }
    }
}

impl Iterator for Spiral {
    type Item = glam::IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.prev_offset;
        let glam::IVec2 { x, y } = self.prev_offset;
        if x < y {
            self.prev_offset[usize::from(x <= -y)] -= 1;
        } else {
            self.prev_offset[usize::from(-x <= y)] += 1;
        }
        if res.x > self.limit || res.y > self.limit {
            None
        } else {
            Some(res + self.center)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const OFFSETS_1: &[glam::IVec2] = &[
        glam::ivec2(0, 0),
        glam::ivec2(0, 1),
        glam::ivec2(-1, 1),
        glam::ivec2(-1, 0),
        glam::ivec2(-1, -1),
        glam::ivec2(0, -1),
        glam::ivec2(1, -1),
        glam::ivec2(1, 0),
        glam::ivec2(1, 1),
    ];

    #[test]
    fn test_loop() {
        let s = Spiral::new(glam::ivec2(1, 1), 1);
        for c in s {
            dbg!(c);
        }
    }

    #[test]
    fn test_zero() {
        let start = glam::ivec2(1, 1);
        let mut s = Spiral::new(start, 0);
        assert_eq!(s.next(), Some(start));
        assert_eq!(s.next(), None);
    }

    #[test]
    fn test_one() {
        let start = glam::ivec2(0, 0);
        let s = Spiral::new(start, 1);
        for c in s {
            assert!(OFFSETS_1.contains(&c))
        }
    }
}
