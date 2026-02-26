use std::collections::{BTreeMap, VecDeque};

use bitvec::{bitvec, vec::BitVec};
use puzzled_core::{
    Entry, Grid, GridState, Line, LinePosition, Position, Solve, impl_solve_for_grid_state,
};

use crate::{ {{ solution }}, LineConstraint, LineValidation, {{ puzzle | pascal_case }} };

#[derive(Debug)]
pub struct {{ puzzle | pascal_case }}State {
    pub inner: GridState<{{ solution }}>,
}

impl_solve_for_grid_state!({{ puzzle | pascal_case }}, {{ solution }});

impl {{ puzzle | pascal_case }}State {
    pub fn new(solutions: Grid<Option<{{ solution }}>>, entries: Grid<Entry<{{ solution }}>>) -> Self {
        Self {
            inner: GridState { solutions, entries },
        }
    }
}

impl From<&{{ puzzle | pascal_case }}> for {{ puzzle | pascal_case }}State {
    fn from({{ puzzle }}: &{{ puzzle | pascal_case }}) -> Self {
        {{ puzzle | pascal_case }}State::new(solutions, entries)
    }
}

impl Solve<{{ puzzle | pascal_case }}> for {{ puzzle | pascal_case }}State {
    type Value = {{ solution }};
    type Position = Position;

    fn solve(&mut self, pos: &Position, solution: {{ solution }}) -> bool {
        let inner = &mut self.inner;
        inner.solve(pos, solution)
    }

    fn reveal(&mut self, _pos: &Position) -> bool {
        true
    }

    fn enter(&mut self, _pos: &Position, _entry: {{ solution }}) -> bool {
        true
    }

    fn check(&mut self, _pos: &Position) -> Option<bool> {
        None
    }

    fn check_all(&mut self) {}

    fn reveal_all(&mut self) {}

    fn try_finalize(&self) -> Result<Grid<{{ solution }}>, Box<dyn std::error::Error>> {
        self.inner.try_finalize()
    }
}
