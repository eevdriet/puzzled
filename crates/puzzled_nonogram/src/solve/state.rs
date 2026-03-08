use std::collections::{BTreeMap, VecDeque};

use bitvec::{bitvec, vec::BitVec};
use derive_more::{Deref, DerefMut};
use puzzled_core::{Entry, Grid, GridState, Line, LinePosition, Timer, impl_solve_for_grid_state};

use crate::{Fill, LineConstraint, LineValidation, Nonogram};

pub(crate) type LineMap<T> = BTreeMap<Line, T>;
pub(crate) type LineMask = BitVec;

#[derive(Debug, Deref, DerefMut)]
pub struct NonogramState {
    #[deref]
    #[deref_mut]
    pub state: GridState<Fill>,

    pub(crate) frontier: VecDeque<Line>,

    pub(crate) validations: LineMap<LineValidation>,
    pub(crate) constraints: LineMap<BTreeMap<Fill, LineConstraint>>,
    pub(crate) masks: LineMap<BTreeMap<Fill, LineMask>>,
}

impl_solve_for_grid_state!(NonogramState, state, Nonogram, Fill);

impl NonogramState {
    pub fn new(solutions: Grid<Option<Fill>>, entries: Grid<Entry<Fill>>, timer: Timer) -> Self {
        Self {
            state: GridState {
                solutions,
                entries,
                timer,
            },
            frontier: VecDeque::default(),
            validations: LineMap::default(),
            constraints: LineMap::default(),
            masks: LineMap::default(),
        }
    }

    pub fn solutions(&self) -> &Grid<Option<Fill>> {
        &self.state.solutions
    }
    pub fn entries(&self) -> &Grid<Entry<Fill>> {
        &self.state.entries
    }
}

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
    pub fn clear(&mut self) {
        self.frontier.clear();
        self.validations.clear();
        self.constraints.clear();
        self.masks.clear();
    }

    fn _set_mask(&mut self, pos: LinePosition, line_len: usize, prev: Fill, curr: Fill) {
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

        let solutions = fills.map_ref(|cell| cell.solution);
        let entries = fills.map_ref(|cell| Entry::new_with_style(cell.solution, cell.style));
        let timer = Timer::default();

        NonogramState::new(solutions, entries, timer)
    }
}

// impl Solve for NonogramState {
//     type Puzzle = Nonogram;
//     type Value = Fill;
//     type Position = Position;
//     type Error = String;
//
//     delegate! {
//         to self.state {
//             fn solve(&mut self, pos: &Self::Position, solution: Self::Value) -> bool;
//             fn enter(&mut self, pos: &Self::Position, entry: Self::Value) -> bool;
//             fn reveal(&mut self, pos: &Self::Position) -> bool;
//             fn check(&mut self, pos: &Self::Position) -> Option<bool>;
//
//             fn reveal_all(&mut self);
//             fn check_all(&mut self);
//
//             fn enter_checked(&mut self, pos: &Self::Position, entry: Self::Value) -> Option<bool>;
//
//             fn guess(&mut self, pos: &Self::Position, guess: Self::Value) -> bool;
//
//             fn guess_checked(&mut self, pos: &Self::Position, guess: Self::Value) -> Option<bool>;
//
//             fn try_finalize(&self) -> Result<Grid<Fill>, Self::Error>;
//         }
//     }
// }
