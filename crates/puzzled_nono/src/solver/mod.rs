mod constraints;
mod validate;

pub use constraints::*;
pub use validate::*;

use std::{
    collections::{HashMap, VecDeque},
    ops::Index,
};

use bitvec::bitvec;

use crate::{Fill, Line, LineMap, LineMask, LinePosition, Nonogram, Position, Result, Rule, Rules};

#[derive(Debug, Default)]
pub struct Solver {
    rules: LineMap<Rule>,

    frontier: VecDeque<Line>,

    validations: LineMap<LineValidation>,
    constraints: LineMap<HashMap<Fill, LineConstraint>>,
    masks: LineMap<HashMap<Fill, LineMask>>,
}

impl Solver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.rules.clear();

        self.frontier.clear();

        self.validations.clear();
        self.constraints.clear();
        self.masks.clear();
    }

    pub fn solve(&mut self, _puzzle: &mut Nonogram) -> Result<bool> {
        Ok(true)
    }

    pub fn get(&self, line: Line) -> Option<&LineValidation> {
        self.validations.get(&line)
    }

    pub fn update_cell(&mut self, puzzle: &mut Nonogram, pos: Position, fill: Fill) {
        // Record the previous fill and set the current
        let prev = puzzle[pos];
        puzzle[pos] = fill;

        let (row_pos, col_pos) = pos.relative();
        let row = row_pos.line;
        let col = col_pos.line;
        let row_len = puzzle.fills().line_len(row);
        let col_len = puzzle.fills().line_len(col);

        // Set the fill in the row and column masks
        self.set_mask(row_pos, row_len, prev, fill);
        self.set_mask(col_pos, col_len, prev, fill);

        // Assure the rule constraints are generated for both lines
        self.generate_rule_constraints(row);
        self.generate_rule_constraints(col);

        // Then validate both lines that the fill affects
        let row_valid = self.validate(puzzle, row);
        self.validations.insert(row, row_valid);

        let col_valid = self.validate(puzzle, col);
        self.validations.insert(col, col_valid);
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

    pub fn insert_rules(&mut self, rules: &Rules) {
        for (r, rule) in rules.rows.iter().enumerate() {
            let row = Line::Row(r);
            self.rules.insert(row, rule.clone());
        }

        for (c, rule) in rules.cols.iter().enumerate() {
            let col = Line::Col(c);
            self.rules.insert(col, rule.clone());
        }
    }
}

impl Index<Line> for Solver {
    type Output = LineValidation;

    fn index(&self, line: Line) -> &Self::Output {
        self.validations
            .get(&line)
            .unwrap_or(&LineValidation::Valid)
    }
}
