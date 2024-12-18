use super::{Coord, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    pub pos: Coord,
    pub dir: Direction,
}

impl Robot {
    pub fn new(start: Coord, direction: Direction) -> Self {
        Self {
            pos: start,
            dir: direction,
        }
    }

    pub fn position(&self) -> Coord {
        self.pos
    }

    pub fn direction(&self) -> Direction {
        self.dir
    }

    pub fn next_pos(&self) -> Coord {
        self.pos + self.dir
    }

    pub fn rotate(&mut self, degrees: i16) {
        self.dir.rotate(degrees);
    }

    pub fn locomote(&mut self) {
        self.pos += self.dir;
    }
}
