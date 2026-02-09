mod cells;
mod checksums;
mod clues;
mod error;
mod extra;
mod grid;
mod header;
mod read;
mod strings;

pub use error::*;
pub(crate) use extra::*;
pub(crate) use grid::*;
pub(crate) use header::*;
pub(crate) use read::*;
pub(crate) use strings::*;

use crate::Puzzle;

pub struct Parser<'a> {
    strict: bool,
    warnings: Vec<Error>,

    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Puzzle> {
        let (puzzle, _) = Self::parse_with_warnings(input, false)?;
        Ok(puzzle)
    }

    pub fn parse_strict(input: &'a [u8]) -> Result<Puzzle> {
        let (puzzle, _) = Self::parse_with_warnings(input, true)?;
        Ok(puzzle)
    }

    pub fn parse_with_warnings(input: &'a [u8], strict: bool) -> Result<(Puzzle, Vec<Error>)> {
        // Parse and validate the main contents of the puzzle
        let mut parser = Self::new(input, strict);

        let header = parser.parse_header()?;
        let grid = parser.parse_grid(header.width, header.height)?;
        let strings = parser.parse_strings(header.clue_count as usize)?;

        parser.validate_checksums(&header, &grid, &strings)?;

        // Derive the puzzle clues and parse extra sections
        let extras = parser.parse_extras(header.width, header.height)?;

        // Build the puzzle with owned data
        let cells = parser.parse_cells(&grid, &extras)?;
        let entries = parser.parse_entries(&grid, &strings.clues)?;
        let puzzle = Puzzle::builder()
            .entries(entries)
            .cells(cells)
            .author(parse_string(strings.author))
            .copyright(parse_string(strings.copyright))
            .notes(parse_string(strings.notes))
            .title(parse_string(strings.title))
            .version(parse_string(header.version))
            .build();

        Ok((puzzle, parser.warnings))
    }

    fn new(input: &'a [u8], strict: bool) -> Self {
        Self {
            strict,
            input,
            warnings: Vec::new(),
            pos: 0,
        }
    }

    pub(crate) fn ok_or_warn<T>(&mut self, result: Result<T>) -> Result<Option<T>> {
        match result {
            // Pass through ok/err with strict mode normally
            Ok(val) => Ok(Some(val)),
            Err(err) if self.strict => Err(err),

            // Warn against errors in non-strict mode
            Err(warning) => {
                self.warnings.push(warning);
                Ok(None)
            }
        }
    }
}
