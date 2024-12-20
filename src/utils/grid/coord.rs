use super::Direction;
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub r: isize,
    pub c: isize,
}

impl Coord {
    pub fn new(r: isize, c: isize) -> Coord {
        Self { r, c }
    }

    pub fn x(&self) -> isize {
        self.c
    }

    pub fn y(&self) -> isize {
        self.r
    }

    pub fn taxicab_distance(&self, rhs: Self) -> usize {
        self.r.abs_diff(rhs.r) + self.c.abs_diff(rhs.c)
    }

    pub fn orthogonal_neighbours(self) -> [Coord; 4] {
        Direction::ORTHOGONAL.map(|dir| self + dir)
    }

    pub fn diagonal_neighbours(self) -> [Coord; 4] {
        Direction::DIAGONAL.map(|dir| self + dir)
    }

    pub fn all_neighbours(self) -> [Coord; 8] {
        Direction::ALL.map(|dir| self + dir)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord::new(self.r + rhs.r, self.c + rhs.c)
    }
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Coord::from(rhs)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.c += rhs.c;
    }
}

impl AddAssign<Direction> for Coord {
    fn add_assign(&mut self, rhs: Direction) {
        *self += Coord::from(rhs);
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord::new(self.r - rhs.r, self.c - rhs.c)
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.c -= rhs.c;
    }
}

impl Mul<isize> for Coord {
    type Output = Coord;

    fn mul(self, rhs: isize) -> Self::Output {
        Coord::new(self.r * rhs, self.c * rhs)
    }
}

impl Mul<Coord> for isize {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord::new(self * rhs.r, self * rhs.c)
    }
}

impl Div<isize> for Coord {
    type Output = Coord;

    fn div(self, rhs: isize) -> Self::Output {
        Coord::new(self.r / rhs, self.c / rhs)
    }
}

impl Rem<isize> for Coord {
    type Output = Coord;

    fn rem(self, rhs: isize) -> Self::Output {
        Coord::new(self.r % rhs, self.c % rhs)
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Coord::new(value.0, value.1)
    }
}

impl From<Coord> for (isize, isize) {
    fn from(value: Coord) -> Self {
        (value.r, value.c)
    }
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Coord::new(-1, 0),
            Direction::NorthEast => Coord::new(-1, 1),
            Direction::East => Coord::new(0, 1),
            Direction::SouthEast => Coord::new(1, 1),
            Direction::South => Coord::new(1, 0),
            Direction::SouthWest => Coord::new(1, -1),
            Direction::West => Coord::new(0, -1),
            Direction::NorthWest => Coord::new(-1, -1),
        }
    }
}
