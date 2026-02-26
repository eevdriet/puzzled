use puzzled_core::{Metadata, Puzzle};

#[derive(Debug, PartialEq, Eq)]
pub struct {{ puzzle | pascal_case }} {
    // State

    // Metadata
    meta: Metadata,
}

impl {{ puzzle | pascal_case}} {
    pub fn meta(&self) -> &Metadata {
        &self.meta
    }
}

impl Puzzle for {{ puzzle | pascal_case }} {
    type Solution = Grid<{{ solution }}>;
}
