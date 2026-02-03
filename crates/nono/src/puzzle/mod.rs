mod error;
mod find;
mod iter;
mod validate;

use bitvec::bitvec;
pub use error::*;
pub use find::*;
pub use iter::*;
pub use validate::*;

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use crate::{Error, Fill, Line, LineMask, LinePosition, Position, Result};

#[derive(Debug, Default)]
pub struct Puzzle {
    // Contents
    rows: u16,
    cols: u16,
    fills: Vec<Fill>,

    // Masks for easy accessing
    masks: HashMap<Line, HashMap<Fill, LineMask>>,
}

impl Puzzle {
    // Constructors
    pub fn new(rows: u16, cols: u16, fills: Vec<Fill>) -> Result<Self> {
        let size = usize::from(rows) * usize::from(cols);
        if fills.len() != size {
            return Err(Error::Puzzle(PuzzleError::SizeMismatch {
                rows,
                cols,
                size,
            }));
        }

        // Create the puzzle
        let mut puzzle = Self {
            rows,
            cols,
            fills,
            masks: HashMap::new(),
        };

        // Generate the fill masks
        for row in 0..rows {
            for col in 0..cols {
                let pos = Position::new(row, col);
                let fill = puzzle[pos];

                puzzle.fill_cell(pos, fill);
            }
        }

        Ok(puzzle)
    }

    pub fn empty(rows: u16, cols: u16) -> Self {
        let size = usize::from(rows) * usize::from(cols);
        let fills = vec![Fill::Blank; size];

        Self::new(rows, cols, fills).unwrap()
    }

    // Iterating
    // Getters
    pub fn size(&self) -> usize {
        usize::from(self.rows) * usize::from(self.cols)
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }
    pub fn cols(&self) -> u16 {
        self.cols
    }

    // Indexing
    fn index(&self, pos: Position) -> usize {
        usize::from(pos.row) * usize::from(self.cols) + usize::from(pos.col)
    }

    // Line
    fn line_len(&self, line: Line) -> u16 {
        match line {
            Line::Row(_) => self.cols,
            Line::Col(_) => self.rows,
        }
    }

    // Setters
    pub fn fill_cell(&mut self, pos: Position, fill: Fill) {
        // Determine previous fill
        let prev = self[pos];

        // Set new fill if different
        if prev == fill {
            return;
        }

        self[pos] = fill;

        let mut set_mask = |pos: LinePosition, prev: Fill, curr: Fill| {
            // Retrieve the masks for the given line
            let line = pos.line;
            let line_len = self.line_len(line) as usize;
            let pos = pos.offset as usize;

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
        };

        let (row_pos, col_pos) = pos.relative();

        set_mask(row_pos, prev, fill);
        set_mask(col_pos, prev, fill);
    }

    // Runs
}

impl<P> Index<P> for Puzzle
where
    P: Into<Position>,
{
    type Output = Fill;

    fn index(&self, pos: P) -> &Self::Output {
        let idx = self.index(pos.into());
        &self.fills[idx]
    }
}

impl<P> IndexMut<P> for Puzzle
where
    P: Into<Position>,
{
    fn index_mut(&mut self, pos: P) -> &mut Self::Output {
        let idx = self.index(pos.into());
        &mut self.fills[idx]
    }
}
