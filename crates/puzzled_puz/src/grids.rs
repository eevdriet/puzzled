use crate::{Context, PuzRead, PuzWrite, format, read, windows_1252_to_char, write};
use puzzled_core::{Grid, GridError};

pub const NON_PLAYABLE_CELL: char = '.';
pub const MISSING_ENTRY_CELL: char = '-';

/// [Grids]((https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#puzzle-layout-and-state)) section
///
/// This section efines the layout of [puzzle](crate::Puz) to be read.
/// Specifically, the following 2 grids are read from the `header.width` and `header.height`:
/// 1.  A *solution* grid containing the [solution](crate::Solution) to each [square](crate::Square)
///     To indicate a [non-playable (black) square](crate::Square::Black), a `b"."` is used.
///     The other squares are the playable [cells](crate::Cell) that the user can put their solutions into.
/// 2.  A *state* grid containing the current [entry](crate::Cell::entry) to each square
///     Note that *even if a user has not yet entered any solutions, a full state grid is read*.
///     Squares that do not yet contain a user entry are indicate with `b"-"`
///
/// As an example, consider the following puzzle and its underlying puzzle grids in binary form:
/// ```
/// use puzzled::crossword::crossword;
///
/// let puzzle = crossword! (
///     [C . .]
///     [A . .]
///     [R O W]
/// );
///
/// // Underlying byte data to represent the puzzle grids
/// // Note that the `crossword!` macro doesn't include user entries
/// let solution = b"C..A..ROW";
/// let state = b"-..-..---";
/// ```
///
/// The crate uses a [`Grid<Square>`](crate::Grid<Square>) to store both the solution and state in a single grid.
#[derive(Debug)]
pub struct Grids {
    pub solution: Grid<u8>,
    pub state: Grid<u8>,

    pub width: u8,
    pub height: u8,
}

impl Grids {
    pub fn validate(&self) -> format::Result<()> {
        let grids = [(&self.state, "puzzle"), (&self.solution, "answer")];

        let err = |kind: GridError| format::Error::Grid(kind);

        for (grid, _) in &grids {
            let len = grid.rows() as u8;

            if len != self.height {
                return Err(err(GridError::InvalidHeight {
                    found: len,
                    expected: self.height,
                }));
            }

            // Check whether the rows have the correct width
            for (r, row) in grid.iter_rows().enumerate() {
                let len = row.count() as u8;

                if len != self.width {
                    return Err(err(GridError::InvalidWidth {
                        row: r as u8,
                        found: len,
                        expected: self.width,
                    }));
                }
            }
        }

        // Check that non-playable squares match in the layout and state
        for ((pos, &solution_square), &state_square) in
            self.solution.iter_indexed().zip(self.state.iter())
        {
            if (solution_square == NON_PLAYABLE_CELL as u8)
                != (state_square == NON_PLAYABLE_CELL as u8)
            {
                return Err(format::Error::CellMismatch {
                    solution_square: windows_1252_to_char(solution_square),
                    state_square: windows_1252_to_char(state_square),
                    row: pos.row as u8,
                    col: pos.col as u8,
                });
            }
        }

        Ok(())
    }
}

/// # Read
impl Grids {
    pub(crate) fn read_from<R>(reader: &mut R, width: u8, height: u8) -> read::Result<Self>
    where
        R: PuzRead,
    {
        let uwidth = width as usize;
        let size = uwidth * usize::from(height);

        let solution = reader.read_vec(size).context("Solution grid")?;
        let solution = Grid::from_vec(solution, uwidth).expect("Read correct length");

        let state = reader.read_vec(size).context("State grid")?;
        let state = Grid::from_vec(state, uwidth).expect("Read correct length");

        Ok(Self {
            solution,
            state,
            width,
            height,
        })
    }
}

/// # Write
impl Grids {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        writer
            .write_all(self.solution.data())
            .context("Solution grid")?;

        writer.write_all(self.state.data()).context("State grid")?;

        Ok(())
    }
}
