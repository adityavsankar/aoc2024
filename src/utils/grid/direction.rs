#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Direction {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ORTHOGONAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const DIAGONAL: [Direction; 4] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];

    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub fn rotate(&mut self, degrees: i16) {
        *self = self.rotated(degrees);
    }

    pub fn rotated(self, degrees: i16) -> Self {
        Self::ALL[(self as i16 + (degrees / 45)).rem_euclid(8) as usize]
    }

    pub fn is_opposite_to(self, other: Direction) -> bool {
        (self as u8).abs_diff(other as u8) == 4
    }

    pub fn is_orthogonal_to(self, other: Direction) -> bool {
        let (s, o) = (self as u8, other as u8);
        (s + 2).rem_euclid(8) == o || (s - 2).rem_euclid(8) == o
    }
}
