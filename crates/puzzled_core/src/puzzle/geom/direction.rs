use std::{fmt, ops};

use crate::{Offset, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

impl ops::Mul<isize> for Direction {
    type Output = Offset;

    fn mul(self, coef: isize) -> Self::Output {
        match self {
            Direction::Up => coef * Offset::UP,
            Direction::Right => coef * Offset::RIGHT,
            Direction::Down => coef * Offset::DOWN,
            Direction::Left => coef * Offset::LEFT,
        }
    }
}
