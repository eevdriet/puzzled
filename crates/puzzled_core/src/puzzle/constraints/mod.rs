mod cells;
mod cmp;
mod line;
mod region;

pub use cells::*;
pub use cmp::*;
pub use line::*;
pub use region::*;

pub enum Constraint {
    Region(RegionConstraint),
    Line(LineConstraint),
    Cells(CellsConstraint),
}

pub trait SatisfiesConstraint {
    fn satisfies_constraint(&self, constraint: &Constraint) -> bool;
}

pub trait SatisfiesRegionConstraint {
    fn satisfies_region_constraint(&self, constraint: &RegionConstraint) -> bool;
}

pub trait SatisfiesLineConstraint {
    fn satisfies_line_constraint(&self, constraint: &LineConstraint) -> bool;
}

pub trait SatisfiesCellsConstraint {
    fn satisfies_cells_constraint(&self, constraint: &CellsConstraint) -> bool;
}
