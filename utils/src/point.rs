use std::{
    iter::Step,
    ops::{Add, AddAssign, Sub},
};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Eq, Hash, PartialEq)]
pub struct Point<T>
where
    T: Sized
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + Eq
        + PartialEq
        + std::hash::Hash,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Default
        + Step
        + Sized
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + Eq
        + PartialEq
        + std::hash::Hash,
{
    pub fn north(&self) -> Self {
        Self {
            x: self.x,
            y: T::backward(self.y, 1),
        }
    }
    pub fn south(&self) -> Self {
        Self {
            x: self.x,
            y: T::forward(self.y, 1),
        }
    }
    pub fn east(&self) -> Self {
        Self {
            x: T::forward(self.x, 1),
            y: self.y,
        }
    }
    pub fn west(&self) -> Self {
        Self {
            x: T::backward(self.x, 1),
            y: self.y,
        }
    }

    pub fn northeast(&self) -> Self {
        Self {
            x: T::forward(self.x, 1),
            y: T::backward(self.y, 1),
        }
    }
    pub fn northwest(&self) -> Self {
        Self {
            x: T::backward(self.x, 1),
            y: T::backward(self.y, 1),
        }
    }
    pub fn southeast(&self) -> Self {
        Self {
            x: T::forward(self.x, 1),
            y: T::forward(self.y, 1),
        }
    }
    pub fn southwest(&self) -> Self {
        Self {
            x: T::backward(self.x, 1),
            y: T::forward(self.y, 1),
        }
    }

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            self.north(),
            self.east(),
            self.south(),
            self.west(),
            self.northeast(),
            self.southeast(),
            self.southwest(),
            self.northwest(),
        ]
    }

    pub fn cardinal(&self) -> Vec<Self> {
        vec![self.north(), self.east(), self.south(), self.west()]
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> Add<Self> for Point<T>
where
    T: Sized
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + Eq
        + PartialEq
        + std::hash::Hash,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T> AddAssign<Self> for Point<T>
where
    T: Sized
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + Eq
        + PartialEq
        + std::hash::Hash,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Add<&Self> for Point<T>
where
    T: Sized
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + Eq
        + PartialEq
        + std::hash::Hash,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Self> for Point<T>
where
    T: Sized
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + Eq
        + PartialEq
        + std::hash::Hash,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Sub<&Self> for Point<T>
where
    T: Sized
        + Copy
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + Eq
        + PartialEq
        + std::hash::Hash,
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;

    #[test]
    fn test_add() {
        let a = Point::new(1, 2);
        let b = Point::new(-1, -1);
        assert_eq!(Point::new(0, 1), a + b);
        assert_eq!(Point::new(0, 1), b + a);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(1, 2);
        let b = Point::new(-1, -1);
        assert_eq!(Point::new(2, 3), a - b);
        assert_eq!(Point::new(-2, -3), b - a);
    }

    #[test]
    fn test_cardinal() {
        let mut cardinal = Point::new(2, 2).cardinal();
        cardinal.sort();
        let mut expected = vec![
            Point::new(2, 1),
            Point::new(1, 2),
            Point::new(3, 2),
            Point::new(2, 3),
        ];
        expected.sort();

        assert_eq!(expected, cardinal);
    }

    #[test]
    fn test_neighbours() {
        let mut neighbours = Point::new(2, 2).neighbours();
        neighbours.sort();
        let mut expected = vec![
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(1, 2),
            Point::new(3, 2),
            Point::new(1, 3),
            Point::new(2, 3),
            Point::new(3, 3),
        ];
        expected.sort();

        assert_eq!(expected, neighbours);
    }
}
