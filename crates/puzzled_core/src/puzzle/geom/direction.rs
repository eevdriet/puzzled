use std::{fmt, ops};

use crate::{Offset, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => 'U',
                Direction::Down => 'D',
                Direction::Left => 'L',
                Direction::Right => 'R',
            }
        )
    }
}

impl ops::Add<Direction> for Position {
    type Output = Option<Self>;

    fn add(self, direction: Direction) -> Self::Output {
        let row = self.row;
        let col = self.col;

        let pos = match direction {
            Direction::Up => Position::new(row.checked_sub(1)?, col),
            Direction::Right => Position::new(row, col + 1),
            Direction::Down => Position::new(row + 1, col),
            Direction::Left => Position::new(row, col.checked_sub(1)?),
        };
        Some(pos)
    }
}

impl ops::Mul<Direction> for isize {
    type Output = Offset;

    fn mul(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => self * Offset::UP,
            Direction::Right => self * Offset::RIGHT,
            Direction::Down => self * Offset::DOWN,
            Direction::Left => self * Offset::LEFT,
        }
    }
}
