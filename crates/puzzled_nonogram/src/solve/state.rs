use std::collections::{BTreeMap, VecDeque};

use bitvec::{bitvec, vec::BitVec};
use puzzled_core::{
    Entry, Grid, GridState, Line, LinePosition, Position, Solve, impl_solve_for_grid_state,
};

use crate::{Fill, LineConstraint, LineValidation, Nonogram};

pub(crate) type LineMap<T> = BTreeMap<Line, T>;
pub(crate) type LineMask = BitVec;

#[derive(Debug)]
pub struct NonogramState {
    pub inner: GridState<Fill>,

    pub(crate) frontier: VecDeque<Line>,

    pub(crate) validations: LineMap<LineValidation>,
    pub(crate) constraints: LineMap<BTreeMap<Fill, LineConstraint>>,
    pub(crate) masks: LineMap<BTreeMap<Fill, LineMask>>,
}

impl_solve_for_grid_state!(Nonogram, Fill);

// pub fn get(&self, line: Line) -> Option<&LineValidation> {
//     self.validations.get(&line)
// }
//
// pub fn update_cell(&mut self, puzzle: &mut Nonogram, pos: Position, fill: Fill) {
//     // Determine current line positions
//     let (row_pos, col_pos) = pos.relative();
//     let row = row_pos.line;
//     let col = col_pos.line;
//     let row_len = puzzle.fills().line_len(row);
//     let col_len = puzzle.fills().line_len(col);
//
//     // Override the previous fill if any and record the current
//     if let Some(prev) = puzzle[pos].entry() {
//         // Set the fill in the row and column masks
//         self.set_mask(row_pos, row_len, *prev, fill);
//         self.set_mask(col_pos, col_len, *prev, fill);
//     }
//
//     puzzle[pos].enter(fill);
//
//     // Assure the rule constraints are generated for both lines
//     self.generate_rule_constraints(row);
//     self.generate_rule_constraints(col);
//
//     // Then validate both lines that the fill affects
//     let row_valid = self.validate(puzzle, row);
//     self.validations.insert(row, row_valid);
//
//     let col_valid = self.validate(puzzle, col);
//     self.validations.insert(col, col_valid);
// }

impl NonogramState {
    pub fn new(solutions: Grid<Option<Fill>>, entries: Grid<Entry<Fill>>) -> Self {
        Self {
            inner: GridState { solutions, entries },
            frontier: VecDeque::default(),
            validations: LineMap::default(),
            constraints: LineMap::default(),
            masks: LineMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.frontier.clear();
        self.validations.clear();
        self.constraints.clear();
        self.masks.clear();
    }

    fn set_mask(&mut self, pos: LinePosition, line_len: usize, prev: Fill, curr: Fill) {
        // Retrieve the masks for the given line
        let line = pos.line;
        let pos = pos.pos;

        let masks = self.masks.entry(line).or_default();

        // Unset the previous fill
        if let Some(mask) = masks.get_mut(&prev) {
            mask.set(pos, false)
        }

        // Do not include blanks in the masks
        if matches!(curr, Fill::Blank) {
            return;
        }

        // Set the current fill
        let empty_mask = bitvec![0; line_len];
        let mask = masks.entry(curr).or_insert(empty_mask);

        mask.set(pos, true);
    }

    pub fn validate_masks(&self, line: Line) -> LineValidation {
        // Get the puzzle masks for the current
        // If none are set (empty line), it is always valid
        let Some(masks) = self.masks.get(&line) else {
            return LineValidation::Valid;
        };

        // Get the line constraints for the current rule
        // If it is unconstrained, the line is always valid
        let Some(constraints) = self.constraints.get(&line) else {
            return LineValidation::Solved;
        };

        tracing::info!("Validate {line:?}");
        tracing::info!("\tMasks: {masks:?}");
        tracing::info!("\tConstraints: {constraints:?}");

        // Verify each of the fills in the line that are currently set
        // Note the .filter to avoid fills that have been previously been set but not currently
        for (&fill, mask) in masks.iter().filter(|(_, mask)| mask.any()) {
            // Invalidate right away if rule doesn't include current fill
            let Some(LineConstraint { required, optional }) = constraints.get(&fill) else {
                tracing::info!("Constraint not found for {fill:?} on {line:?}");
                return LineValidation::InvalidFill(fill);
            };

            // Fill is invalid if it's not placed on one of the optional cells
            if !(optional.clone() & mask).any() {
                tracing::info!("Invalid fill for {line:?}");
                tracing::info!("\tRequired bits: {required}");
                tracing::info!("\tOptional bits: {optional}");
                tracing::info!("\tSet:           {mask}");

                return LineValidation::InvalidFill(fill);
            }
        }

        LineValidation::Valid
    }
}

impl From<&Nonogram> for NonogramState {
    fn from(nonogram: &Nonogram) -> Self {
        let fills = nonogram.fills();

        let solutions = fills.map_ref(|cell| cell.solution.clone());
        let entries = fills.map_ref(|cell| {
            let mut entry = Entry::default_with_style(cell.style);

            if let Some(ref solution) = cell.solution {
                entry.enter(solution.clone());
            }

            entry
        });

        NonogramState::new(solutions, entries)
    }
}

impl Solve<Nonogram> for NonogramState {
    type Value = Fill;
    type Position = Position;

    fn solve(&mut self, pos: &Position, solution: Fill) -> bool {
        let inner = &mut self.inner;
        inner.solve(pos, solution)
    }

    fn reveal(&mut self, _pos: &Position) -> bool {
        true
    }

    fn enter(&mut self, _pos: &Position, _entry: Fill) -> bool {
        true
    }

    fn check(&mut self, _pos: &Position) -> Option<bool> {
        None
    }

    fn check_all(&mut self) {}

    fn reveal_all(&mut self) {}

    fn try_finalize(&self) -> Result<Grid<Fill>, Box<dyn std::error::Error>> {
        self.inner.try_finalize()
    }
}
