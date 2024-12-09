#![allow(dead_code)]

use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.r + rhs.r, self.c + rhs.c)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.r - rhs.r, self.c - rhs.c)
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point::new(self.r * rhs, self.c * rhs)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn get(&self, point: Point) -> Option<&T> {
        if !self.contains(point) {
            return None;
        }
        self.cells.get(self.point_to_index(point))
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut T> {
        if !self.contains(point) {
            return None;
        }
        let index = self.point_to_index(point);
        self.cells.get_mut(index)
    }

    pub fn contains(&self, point: Point) -> bool {
        (0..self.height as isize).contains(&point.r) && (0..self.width as isize).contains(&point.c)
    }

    pub fn row(&self, r: usize) -> &[T] {
        let r = self.height * r;
        &self.cells[r..r + self.width]
    }

    fn point_to_index(&self, point: Point) -> usize {
        point.r as usize * self.width + point.c as usize
    }
}

impl From<&str> for Grid<u8> {
    fn from(value: &str) -> Self {
        let rows: Vec<&str> = value.trim().lines().collect();
        let height = rows.len();
        let width = rows.get(0).map_or(0, |line| line.len());
        assert!(
            rows.iter().all(|row| row.len() == width),
            "Rows have unequal lengths"
        );
        let cells = rows.into_iter().flat_map(|row| row.bytes()).collect();
        Self {
            height,
            width,
            cells,
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.cells[self.point_to_index(index)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let index = self.point_to_index(index);
        &mut self.cells[index]
    }
}
