use super::Coord;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T: Copy + PartialEq> {
    height: usize,
    width: usize,
    cells: Vec<T>,
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn new(height: usize, width: usize, default_value: T) -> Self {
        Self {
            height,
            width,
            cells: vec![default_value; height * width],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        if !self.contains(coord) {
            return None;
        }
        self.cells.get(self.coord_to_index(coord))
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if !self.contains(coord) {
            return None;
        }
        let index = self.coord_to_index(coord);
        self.cells.get_mut(index)
    }

    pub fn contains(&self, coord: Coord) -> bool {
        (0..self.height as isize).contains(&coord.r) && (0..self.width as isize).contains(&coord.c)
    }

    pub fn row(&self, r: usize) -> &[T] {
        let r = self.width * r;
        &self.cells[r..r + self.width]
    }

    fn coord_to_index(&self, coord: Coord) -> usize {
        coord.r as usize * self.width + coord.c as usize
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        Coord::new((index / self.width) as isize, (index % self.width) as isize)
    }

    pub fn iter_with_coords(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, cell)| (self.index_to_coord(i), cell))
    }

    pub fn position(&self, target: T) -> Option<Coord> {
        self.iter_with_coords()
            .find(|&(_, cell)| *cell == target)
            .map(|(coord, _)| coord)
    }

    pub fn fill(&mut self, value: T) {
        self.cells.fill(value);
    }
}

impl<T: Copy + PartialEq> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.cells[self.coord_to_index(index)]
    }
}

impl<T: Copy + PartialEq> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let index = self.coord_to_index(index);
        &mut self.cells[index]
    }
}

impl<T: Copy + PartialEq> IntoIterator for Grid<T> {
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

impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.coord_to_index(Coord::new(row as isize, col as isize));
                let ch = self.cells[index] as char;
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
