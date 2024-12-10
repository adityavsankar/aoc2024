#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
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
        *self = Self::ALL[((*self as i16 + (degrees / 45)) % 8) as usize];
    }
}
