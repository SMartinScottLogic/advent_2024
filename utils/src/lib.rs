#![feature(step_trait)]
mod direction;
pub mod graph;
pub mod math;
mod matrix;
mod point;
mod runner;
mod solution;
mod sparsegrid;

pub use direction::Direction;
pub use matrix::Matrix;
pub use point::Point;
pub use runner::{log_init, run, BaseName};
pub use solution::{load, Solution};
pub use sparsegrid::SparseGrid;

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
