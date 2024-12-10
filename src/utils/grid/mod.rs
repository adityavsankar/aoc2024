#![allow(dead_code)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

mod coord;
mod direction;
#[allow(clippy::module_inception)]
mod grid;
mod robot;

pub use coord::Coord;
pub use direction::Direction;
pub use grid::Grid;
pub use robot::Robot;
