mod state;

pub use state::*;

use puzzled_core::{Grid, Solve, Solver};

use crate::{ {{ puzzle | pascal_case }} };

#[derive(Debug, Default)]
pub struct {{ puzzle | pascal_case }}Solver {}

impl Solver<{{ puzzle | pascal_case }}> for {{ puzzle | pascal_case }}Solver {
    fn solve<S>(&mut self, _puzzle: &{{ puzzle | pascal_case }}, _state: &mut S) -> Grid<Fill>
    where
        S: Solve<{{ puzzle | pascal_case }}>,
    {
        Grid::new(0, 0).expect("Temporary")
    }
}

impl {{ puzzle | pascal_case }}Solver {
    pub fn new() -> Self {
        Self::default()
    }
}
