#![feature(step_trait)]

pub mod grid;
pub mod point;

//mod direction;
//mod fixedgrid;
pub mod graph;
pub mod math;
//mod matrix;
//mod picture;
//mod point;
mod region;
mod runner;
mod solution;
//mod sparsegrid;

//pub use point::Direction;
//pub use fixedgrid::FixedGrid;
//pub use matrix::Matrix;
//pub use picture::Picture;
//pub use point::Point;
pub use runner::{log_init, run, BaseName};
pub use solution::{load, Solution};
//pub use sparsegrid::SparseGrid;

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
