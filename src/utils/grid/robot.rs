use super::{Direction, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    pos: Point,
    dir: Direction,
}

impl Robot {
    pub fn new(start: Point, direction: Direction) -> Self {
        Self {
            pos: start,
            dir: direction,
        }
    }

    pub fn position(&self) -> Point {
        self.pos
    }

    pub fn direction(&self) -> Direction {
        self.dir
    }

    pub fn next_pos(&self) -> Point {
        self.pos + Point::from(self.dir)
    }

    pub fn rotate(&mut self, degrees: i16) {
        self.dir.rotate(degrees);
    }

    pub fn locomote(&mut self) {
        self.pos += Point::from(self.dir);
    }
}
