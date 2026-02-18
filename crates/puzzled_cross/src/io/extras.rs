use std::collections::BTreeMap;

use crate::{CellStyle, Puzzle, SizeCheck, Square, format};
use puzzled_core::{Grid, Position, Timer};

pub(crate) type Grbs = Grid<u8>;
pub(crate) type Rtbl = BTreeMap<u8, String>;
pub(crate) type Ltim = Timer;
pub(crate) type Gext = Grid<CellStyle>;

#[derive(Debug, Default)]
pub struct Extras {
    /// The GRBS section contains one byte per square of the board.
    /// Each byte indicates whether or not the corresponding square is a rebus.
    /// Possible values are
    /// - `0` to indicate a non-rebus square
    /// - `1+` to indicate a rebus square, the solution for which is given by the entry with key `n` in the [RTBL] section
    pub grbs: Option<Grbs>,

    /// The RTBL section contains a string that represents the solutions for any rebus squares
    /// The solutions are separated by semi-colons and contain the square number and actual rebus
    /// For example, "0:HEART;1:DIAMOND;17:CLUB;23:SPADE" represents 4 rebuses at squares 0, 1, 17 and 23
    pub rtbl: Option<Rtbl>,

    /// The LTIM section contains information on
    /// Specifically, two strings are stored which are separated by a comma.
    /// The former represents how much time the solver has used and the latter whether the timer is running or stopped.
    /// A value of `0` represents the timer running and `1` that the timer is stopped.
    pub ltim: Option<Ltim>,

    /// The GEXT section contains one byte per square of the board
    /// Each byte represents a bitmask indicating that some style attributes are set
    /// The meaning for the following four bits are known:
    /// - `0x10` means that the square was previously marked incorrect
    /// - `0x20` means that the square is currently marked incorrect
    /// - `0x40` means that the contents of the square were given
    /// - `0x80` means that the square is circled
    pub gext: Option<Gext>,
}

impl Extras {
    pub(crate) fn from_puzzle(puzzle: &Puzzle) -> format::Result<Self> {
        puzzle.squares().check_size()?;

        let mut extras = Extras::default();

        // GRBS / RTBL
        if puzzle.iter_cells().any(|cell| cell.is_rebus()) {
            let mut rebuses: BTreeMap<u8, String> = BTreeMap::new();
            let mut num = 0;

            let squares: Vec<_> = puzzle
                .iter()
                .map(|square| match square {
                    Square::White(cell) if cell.is_rebus() => {
                        num += 1;
                        rebuses.insert(num, cell.solution().to_string());

                        num
                    }
                    _ => 0,
                })
                .collect();

            extras.grbs = Some(Grid::from_vec(squares, puzzle.cols()).expect("correct dimensions"));
            extras.rtbl = Some(rebuses);
        }

        // LTIM
        extras.ltim = Some(puzzle.timer());

        // GEXT
        if !puzzle.iter_cells().all(|cell| cell.style().is_empty()) {
            let styles: Vec<_> = puzzle
                .iter()
                .map(|square| match square {
                    Square::Black => CellStyle::default(),
                    Square::White(cell) => cell.style(),
                })
                .collect();

            extras.gext = Some(Grid::from_vec(styles, puzzle.cols()).expect("correct dimensions"));
        }

        Ok(extras)
    }
}

impl Extras {
    pub fn get_rebus(&self, pos: Position) -> Option<&String> {
        let (Some(grbs), Some(rtbl)) = (&self.grbs, &self.rtbl) else {
            return None;
        };

        let rebus = grbs.get(pos)?;
        rtbl.get(rebus)
    }

    pub fn get_style(&self, pos: Position) -> CellStyle {
        match &self.gext {
            None => CellStyle::default(),
            Some(gext) => *gext.get(pos).unwrap_or(&CellStyle::default()),
        }
    }
}
