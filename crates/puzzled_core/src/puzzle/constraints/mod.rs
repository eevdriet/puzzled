mod cells;
mod line;
mod ord;

pub use cells::*;
pub use line::*;
pub use ord::*;

pub enum Constraint {
    Line(LineConstraint),
    Cells(CellsConstraint),
}
