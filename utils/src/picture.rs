use std::ops::{Deref, DerefMut};

use crate::FixedGrid;

#[derive(Debug)]
pub struct Picture {
    inner: FixedGrid<usize>,
}
impl Picture {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        Self {
            inner: FixedGrid::new(max_x, max_y),
        }
    }

    pub fn clumpiness(&self) -> f64 {
        let mut count = 0;
        let mut clumped = 0;
        for ((px, py), c) in self.inner.iter() {
            if c != 0 {
                count += 1;
                let mut is_clumped = false;
                for [dx, dy] in [[0, 1], [1, 0], [0, -1], [-1, 0]] {
                    if let Some(c) = self.inner.get(px + dx, py + dy) {
                        if *c != 0 {
                            is_clumped = true;
                            break;
                        }
                    }
                }
                if is_clumped {
                    clumped += 1;
                }
            }
        }
        (clumped as f64) / (count as f64)
    }
}
impl Deref for Picture {
    type Target = FixedGrid<usize>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Picture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
