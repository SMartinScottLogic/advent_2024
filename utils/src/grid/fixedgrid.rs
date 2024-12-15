#[derive(Debug, Clone)]
pub struct FixedGrid<T> {
    max_x: usize,
    max_y: usize,
    data: Vec<T>,
}
impl<T> FixedGrid<T>
where
    T: Default,
{
    pub fn new(max_x: usize, max_y: usize) -> Self {
        let mut data = Vec::new();
        data.resize_with(max_x * max_y, T::default);
        Self { max_x, max_y, data }
    }
}

impl<T> FixedGrid<T> {
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if self.in_bounds(x, y) {
            let index = self.index(x, y);
            self.data.get(index)
        } else {
            None
        }
    }

    #[must_use]
    pub fn set_checked(&mut self, x: isize, y: isize, value: T) -> Option<()> {
        if !self.in_bounds(x, y) {
            None
        } else {
            let index = self.index(x, y);
            match self.data.get_mut(index) {
                Some(v) => *v = value,
                None => panic!(
                    "index failure: (x: {}, y: {}, max_x: {}, max_y: {}, index: {})",
                    x, y, self.max_x, self.max_y, index
                ),
            }
            Some(())
        }
    }

    pub fn set(&mut self, x: isize, y: isize, value: T) {
        if self.in_bounds(x, y) {
            let index = self.index(x, y);
            match self.data.get_mut(index) {
                Some(v) => *v = value,
                None => panic!(
                    "index failure: (x: {}, y: {}, max_x: {}, max_y: {}, index: {})",
                    x, y, self.max_x, self.max_y, index
                ),
            }
        }
    }

    pub fn max_x(&self) -> usize {
        self.max_x
    }

    pub fn max_y(&self) -> usize {
        self.max_y
    }

    fn index(&self, x: isize, y: isize) -> usize {
        let x = x as usize;
        let y = y as usize;
        x + y * self.max_x
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.max_x.try_into().unwrap() && y >= 0 && y < self.max_y.try_into().unwrap()
    }
}

impl<T> FixedGrid<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct Iter<'a, T> {
    grid: &'a FixedGrid<T>,
    x: isize,
    y: isize,
}
impl<T> Iterator for Iter<'_, T>
where
    T: Default + Clone,
{
    type Item = ((isize, isize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.in_bounds(self.x, self.y) {
            let cx = self.x;
            let cy = self.y;
            self.x += 1;
            if self.x >= self.grid.max_x as isize {
                self.x = 0;
                self.y += 1;
            }
            self.grid
                .get(self.x, self.y)
                .map(|v| ((cx, cy), v.to_owned()))
        } else {
            None
        }
    }
}
