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
        //panic!("Broken - do not use");
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
            self.grid.get(cx, cy).map(|v| ((cx, cy), v.to_owned()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn manual_iteration() {
        let mut grid = FixedGrid::new(20, 20);
        for y in 0..grid.max_y() {
            for x in 0..grid.max_x() {
                grid.set(x as isize, y as isize, x + y * 100);
            }
        }
        for y in 0..grid.max_y() {
            for x in 0..grid.max_x() {
                let expected = x + y * 100;
                let actual = grid.get(x as isize, y as isize);
                assert!(actual.is_some());
                assert_eq!(&expected, actual.unwrap());
            }
        }
    }

    #[test]
    fn iterator() {
        let mut grid = FixedGrid::new(20, 20);
        for y in 0..grid.max_y() {
            for x in 0..grid.max_x() {
                grid.set(x as isize, y as isize, x + y * 100);
            }
        }
        for ((px, py), value) in grid.iter() {
            let expected = px + py * 100;
            assert_eq!(value, expected as usize);
            assert_eq!(expected as usize, *grid.get(px, py).unwrap());
        }
    }
}
