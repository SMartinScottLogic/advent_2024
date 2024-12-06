use std::fmt::{Debug, Formatter, Result};

#[derive(Default)]
pub struct DenseGrid<T> {
    data: Vec<Vec<T>>,
}
impl <T> Debug for DenseGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("DenseGrid")
        .field("rows", &self.data.len())
        .field("cols", &self.data[0].len())
        .finish()
    }
}
impl <T> DenseGrid<T> {
    pub fn push(&mut self, row: Vec<T>) {
        self.data.push(row);
    }

    pub fn cells(&self) -> impl Iterator<Item=(usize, usize, &T)> {
        self.data.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, c)))
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 {
            None
        } else {
            self.data.get(y as usize).and_then(|row| row.get(x as usize))
        }
    }
}