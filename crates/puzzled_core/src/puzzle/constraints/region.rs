use std::collections::HashSet;

use crate::Position;

pub struct RegionConstraint {
    pub region: HashSet<Position>,
    pub kind: RegionConstraintKind,
}

pub enum RegionConstraintKind {
    Sum(usize), // Killer Sudoku
    NotEqual,
}
