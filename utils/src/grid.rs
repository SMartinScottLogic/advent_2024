use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    iter::Step,
    ops::{Add, AddAssign, RangeInclusive, Sub},
};

use crate::Point;

#[derive(Debug, Clone)]
pub struct Range<T> {
    pub x: RangeInclusive<T>,
    pub y: RangeInclusive<T>,
}
impl<T> Range<T>
where
    T: Default,
{
    fn new() -> Self {
        Self {
            x: T::default()..=T::default(),
            y: T::default()..=T::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid<T, V>
where
    V: Default
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Eq
        + PartialEq
        + Hash,
{
    data: HashMap<Point<V>, T>,
    range: Range<V>,
}
impl<T, V> Grid<T, V>
where
    T: Default + Display + Clone,
    V: Default
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Eq
        + PartialEq
        + Hash,
{
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            range: Range::new(),
        }
    }
}
impl<T, V> Default for Grid<T, V>
where
    T: Default + Display + Clone,
    V: Default
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Eq
        + PartialEq
        + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, V> Grid<T, V>
where
    T: Default + Display + Clone,
    V: Default
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Eq
        + PartialEq
        + Hash,
{
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, point: &Point<V>) -> Option<&T> {
        self.data.get(point)
    }

    pub fn dimensions(&self) -> &Range<V> {
        &self.range
    }

    pub fn min_x(&self) -> &V {
        self.range.x.start()
    }

    pub fn max_x(&self) -> &V {
        self.range.x.end()
    }

    pub fn min_y(&self) -> &V {
        self.range.y.start()
    }

    pub fn max_y(&self) -> &V {
        self.range.y.end()
    }

    pub fn display(&self)
    where
        V: Display + Step,
    {
        self.display_with_mapping(|v| format!("{v}"));
    }
    pub fn display_with_mapping<F>(&self, mapping: F)
    where
        F: Fn(T) -> String,
        V: Display + Step,
    {
        for y in self.range.y.clone() {
            let mut line = String::new();
            line.push_str(&format!("{} ", y));
            for x in self.range.x.clone() {
                let v = match self.get(&Point::new(x, y)) {
                    Some(v) => (*v).to_owned(),
                    None => T::default(),
                };
                let v = mapping(v);
                line.push_str(&v);
            }
            println!("{line}");
        }
    }
}
