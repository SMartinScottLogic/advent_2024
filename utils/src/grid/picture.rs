use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use super::{FixedGrid, SparseGrid};

#[derive(Debug)]
pub struct Picture<T> {
    inner: FixedGrid<T>,
}
impl<T> Picture<T>
where
    T: Default + Clone,
{
    pub fn new(max_x: usize, max_y: usize) -> Self {
        Self {
            inner: FixedGrid::<T>::new(max_x, max_y),
        }
    }
}

impl<T> Picture<T>
where
    T: Default,
{
    pub fn display_with_mapping<'a, F>(&'a self, mapping: F)
    where
        F: Fn(&T) -> &'a str,
    {
        for y in 0..self.inner.max_y() {
            let mut line = String::new();
            line.push_str(&format!("{:04} ", y));
            for x in 0..self.inner.max_x() {
                let v = match self.get(x as isize, y as isize) {
                    Some(v) => v,
                    None => &T::default(),
                };
                line.push_str(mapping(v));
            }
            println!("{line}");
        }
    }
}

impl<T> Picture<T> {
    pub fn display_with_mapping_and_default<F>(&self, mapping: F, default: &T)
    where
        F: Fn(&T) -> &str,
    {
        for y in 0..self.inner.max_y() {
            let mut line = String::new();
            line.push_str(&format!("{:04} ", y));
            for x in 0..self.inner.max_x() {
                let v = match self.get(x as isize, y as isize) {
                    Some(v) => v,
                    None => default,
                };
                let v = mapping(v);
                line.push_str(v);
            }
            println!("{line}");
        }
    }
}

impl<T> Deref for Picture<T> {
    type Target = FixedGrid<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T> DerefMut for Picture<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> From<FixedGrid<T>> for Picture<T> {
    fn from(inner: FixedGrid<T>) -> Self {
        Self { inner }
    }
}

impl<T> From<SparseGrid<T, usize>> for Picture<T>
where
    T: Clone + Copy + Default + Display,
{
    fn from(value: SparseGrid<T, usize>) -> Self {
        let max_x = value.max_x();
        let max_y = value.max_y();
        let mut inner: FixedGrid<T> = FixedGrid::new(*max_x, *max_y);
        for (point, value) in value.iter() {
            inner
                .set_checked(
                    point.x().try_into().unwrap(),
                    point.y().try_into().unwrap(),
                    *value,
                )
                .unwrap();
        }
        Self { inner }
    }
}
