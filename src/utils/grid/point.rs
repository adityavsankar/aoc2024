use super::Direction;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub r: isize,
    pub c: isize,
}

impl Point {
    pub fn new(r: isize, c: isize) -> Point {
        Self { r, c }
    }

    pub fn manhattan_distance(&self, rhs: Self) -> usize {
        self.r.abs_diff(rhs.r) + self.c.abs_diff(rhs.c)
    }

    pub fn orthogonal_neighbours(self) -> [Point; 4] {
        Direction::ORTHOGONAL.map(|dir| self + Point::from(dir))
    }

    pub fn diagonal_neighbours(self) -> [Point; 4] {
        Direction::DIAGONAL.map(|dir| self + Point::from(dir))
    }

    pub fn all_neighbours(self) -> [Point; 8] {
        Direction::ALL.map(|dir| self + Point::from(dir))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.r + rhs.r, self.c + rhs.c)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.c += rhs.c;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.r - rhs.r, self.c - rhs.c)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.c -= rhs.c;
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point::new(self.r * rhs, self.c * rhs)
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Point::new(-1, 0),
            Direction::NorthEast => Point::new(-1, 1),
            Direction::East => Point::new(0, 1),
            Direction::SouthEast => Point::new(1, 1),
            Direction::South => Point::new(1, 0),
            Direction::SouthWest => Point::new(1, -1),
            Direction::West => Point::new(0, -1),
            Direction::NorthWest => Point::new(-1, -1),
        }
    }
}
