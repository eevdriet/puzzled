use crate::{Ordering, Position};

pub struct CellsConstraint {
    pub lhs: Position,
    pub rhs: Position,
    pub kind: CellsConstraintKind,
    pub is_symmetric: bool,
}

pub enum CellsConstraintKind {
    Parity,
    Double,
    Ordering(Ordering),
}
