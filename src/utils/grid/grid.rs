use super::Point;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    height: usize,
    width: usize,
    cells: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

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

    fn index_to_point(&self, index: usize) -> Point {
        Point::new((index / self.width) as isize, (index % self.width) as isize)
    }

    pub fn iter_with_coords(&self) -> impl Iterator<Item = (Point, &T)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, cell)| (self.index_to_point(i), cell))
    }
}

impl<T: Copy> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.cells[self.point_to_index(index)]
    }
}

impl<T: Copy> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let index = self.point_to_index(index);
        &mut self.cells[index]
    }
}

impl<T: Copy> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl From<&str> for Grid<u8> {
    fn from(value: &str) -> Self {
        let rows: Vec<&str> = value.trim().lines().collect();
        let height = rows.len();
        let width = rows.first().map_or(0, |line| line.len());
        assert_ne!(height, 0, "Cannot have 0 height grid");
        assert_ne!(width, 0, "Cannot have 0 width grid");
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
