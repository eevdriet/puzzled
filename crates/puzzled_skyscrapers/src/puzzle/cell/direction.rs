use std::{fmt, ops};

use puzzled_core::{Line, LineSegment, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    type Output = LineSegment;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => LineSegment::new(Line::Col(self.row), ..self.row + 1),
            Direction::Down => LineSegment::new(Line::Col(self.row), self.row..),
            Direction::Left => LineSegment::new(Line::Row(self.col), ..self.col + 1),
            Direction::Right => LineSegment::new(Line::Row(self.col), ..self.row),
        }
    }
}
