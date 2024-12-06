use strum::IntoEnumIterator;
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, EnumIter)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Direction {
    pub fn iter() -> DirectionIter {
        <Self as IntoEnumIterator>::iter()
    }
}
