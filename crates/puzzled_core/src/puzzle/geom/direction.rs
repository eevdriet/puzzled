use std::{fmt, ops};

use crate::{Offset, Position};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up = 0,

    #[default]
    Right = 1,
    Down = 2,
    Left = 3,
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

impl ops::Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        let idx = (self as usize + rhs as usize) & 3;
        Self::ALL[idx]
    }
}

impl ops::AddAssign<Direction> for Direction {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
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

impl ops::Sub<Direction> for Position {
    type Output = Option<Self>;

    fn sub(self, direction: Direction) -> Self::Output {
        self + !direction
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

impl ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}
