//! Defines all functionality for reading, deserializing and parsing [*.puz files][PUZ google spec]
//!
//! # Usage
//! # Errors and warnings
//! # Parsing process
//! This crate tries to following the [Across Lite specification][PUZ google spec] as closely as possible to parse `*.puz` files from binary data.
//! Note that a [reformatted specification][PUZ spec] which may be easier to read from, as it provides the full specification in Markdown.
//!
//! In each of the following sections, we list the elements that are parsed:
//! - White elements are directly used to define the resulting [puzzle](Puzzle).
//! - <span style="color:yellow">Yellow</span> elements are used for [validating checksums](self#validating-checksums) and checking the byte integrity of the `*.puz` data.
//! - <span style="color:gray">Gray</span> elements are currently ignored
//!
//! ## Header
//! | Component  | Length | Type | Description |
//! |------------|--------|------|-------------|
//! | <span style="color:yellow">Checksum</span>   | 2      | u16  | Overall [file checksum](self#file) |
//! | <span style="color:yellow">File Magic</span> | 12     | str  | NUL-terminated constant string: `b"ACROSS&DOWN\0"` |
//! | <span style="color:yellow">CIB Checksum</span>          | 2      | u16  | [CIB checksum](self#cib) |
//! | <span style="color:yellow">Masked Low Checksums</span>  | 4      | u32  | A set of low [masked checksums](self#masked-regions) |
//! | <span style="color:yellow">Masked High Checksums</span> | 4      | u32  | A set of high [masked checksums](self#masked-regions) |
//! | Version String(?)  | 4      | str  | e.g. "1.2\0" |
//! | <span style="color:gray">Reserved1C(?)</span>      | 2      | u16  | In many files, this is uninitialized memory |
//! | <span style="color:gray">Scrambled Checksum</span> | 2      | u16  | In scrambled puzzles, a checksum of the real solution (details below) |
//! | Width              | 1      | u8   | The width of the board |
//! | Height             | 1      | u8   | The height of the board |
//! | # of Clues         | 2      | u16  | The number of clues for this board |
//! | <span style="color:gray">Unknown Bitmask</span>    | 2      | u16  | A bitmask. Operations unknown. |
//! | <span style="color:gray">Scrambled Tag</span>      | 2      | u16  | 0 for unscrambled puzzles. Nonzero (often 4) for scrambled puzzles. |
//!
//! ## Strings
//! The [strings](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#strings-section) includes the "metadata" for the [puzzle](Puzzle), such as its title and author.
//! It also includes the text for every [clue](crate::Clue) that should be attached to the puzzle.
//! The pseudo-code of how to do so is given [here](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#clue-assignment) and the crate implementation is found in [`Puzzle::position_clues`].
//!
//! | Component | Length | Type | Example |
//! |:------------|:-------------------|:---|:---|
//! | Title       | ? | str | Theme: .PUZ format |
//! | Author      | ? | str | J. Puz / W. Shortz |
//! | Copyright   | ? | str | (c) 2007 J. Puz    |
//! | Clue `#1`   | ? | str | Cued, in pool      |
//! | ...         | ... | ... | ... |
//! | Clue `#n`   | ? | str | Quiet              |
//! | Notes       | ? | str | http://mywebsite   |
//! ## Puzzle grid
//!
//! ## Validating checksums
//! The main validation technique for `*.puz` files is to *match given checksums with region checksums*.
//! Every puzzle contains 3 given checksums in its [header](self#header) that need to be matched.
//! These are explained in the sections below together with a rough outline of how to calculate and validate them.
//! Mainly, it comes down to repeatedly finding the checksum for a given byte-region:
//! ```no_run
//! fn find_region_checksum(region: &[u8], start: u16) -> u16 {
//!     let mut checksum = start;
//!
//!     for &byte in region {
//!         if checksum & 1 != 0 {
//!             checksum = (checksum >> 1) + 0x8000;
//!         } else {
//!             checksum >>= 1;
//!         }
//!
//!         checksum = checksum.wrapping_add(byte as u16);
//!     }
//!
//!     checksum
//! }
//! ```
//! When [`Parser::parse_strict`] is used, all checksums need to be valid in order to successfully parse a [puzzle](Puzzle).
//! Otherwise, the user is [warned](Warning) against invalid or missing checksums.
//!
//! ### CIB
//! The first checksum is the **CIB** checksum, which is specified in the [header](self#header).
//! We need to validate it against the bytes that define the `width` and `height` of the puzzle:
//! ```compile_fail
//! let cib_checksum = find_region_checksum(header.width_height_region, 0);
//! assert_eq!(cib_checksum, header.cib_checksum);
//! ```
//!
//! ### File
//! Next is the **file checksum** checks all data used for the [puzzle](Puzzle), i.e. both [puzzle grids](self#puzzle-grid) and all [strings](self#strings).
//! Below is a rough outline of how it is validated in the parser
//! ```compile_fail
//! fn find_strings_checksum(strings: &Strings, start: u16) -> u16 {
//!     let mut checksum = start;
//!
//!     if strings.title.len() > 1 {
//!         checksum = find_region_checksum(strings.title, checksum);
//!     }
//!     if strings.author.len() > 1 {
//!         checksum = find_region_checksum(strings.author, checksum);
//!     }
//!     if strings.copyright.len() > 1 {
//!         checksum = find_region_checksum(strings.copyright, checksum);
//!     }
//!
//!     for clue in strings.clues {
//!         checksum = find_region_checksum(&clue[..clue.len() - 1], checksum);
//!     }
//!     
//!     if strings.notes.len() > 1 {
//!         checksum = find_region_checksum(strings.notes, checksum);
//!     }
//!
//!     checksum
//! }
//!
//! let file_checksum = {
//!     let mut checksum = header.cib_checksum;
//!
//!     checksum = find_region_checksum(grid.solution_region, checksum);
//!     checksum = find_region_checksum(grid.state_region, checksum);
//!     checksum = find_strings_checksum(strings, checksum);
//!
//!     checksum
//! };
//!
//! assert_eq!(file_checksum, header.file_checksum);
//! ```
//!
//! ### Masked regions
//! Finally we need to validate the [masked regions](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#masked-checksums) given in the [header](self#header).
//! It comes down the calculating the checksums we've seen before and validating them with the `header.mask_checksums`.
//! To do so, amusingly we use the phrase `I CHEATED` and then some XOR-ing and bit shifting.
//!
//! Below is a rough outline of how it is validated in the parser
//! ```compile_fail
//! let cib_checksum = find_region_checksum(header.cib_region, 0);
//! let sol_checksum = find_region_checksum(grid.solution_region, 0);
//! let state_checksum = find_region_checksum(grid.state_region, 0);
//! let strs_checksum = find_strings_checksum(strings, 0);
//!
//! assert_eq!(header.mask_checksums[0], b"I" ^ (cib_checksum & 0xFF));
//! assert_eq!(header.mask_checksums[1], b"C" ^ (sol_checksum & 0xFF));
//! assert_eq!(header.mask_checksums[2], b"H" ^ (state_checksum & 0xFF));
//! assert_eq!(header.mask_checksums[3], b"E" ^ (strs_checksum & 0xFF));
//! assert_eq!(header.mask_checksums[4], b"A" ^ ((cib_checksum & 0xFF00) >> 8));
//! assert_eq!(header.mask_checksums[5], b"T" ^ ((sol_checksum & 0xFF00) >> 8));
//! assert_eq!(header.mask_checksums[6], b"E" ^ ((state_checksum & 0xFF00) >> 8));
//! assert_eq!(header.mask_checksums[7], b"D" ^ ((strs_checksum & 0xFF00) >> 8));
//! ```
//!
//! ## Extra sections
//! When [`Parser::parse_strict`] is used, each extra section must have a valid header (GRBS, TLBR, LTIM or GEXT) and contents.
//! Otherwise, the user is [warned](Warning) against invalid or (partially) missing sections.
//!
//! [puzzled]: crate
//! [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki

mod checksums;
mod clues;
mod error;
mod extra;
mod grid;
mod header;
mod read;
mod squares;
mod strings;

use std::{ops::Range, str::Lines};

pub use error::*;
pub(crate) use extra::*;
pub(crate) use grid::*;
pub(crate) use header::*;
pub(crate) use read::*;
pub(crate) use strings::*;

use crate::Puzzle;

#[derive(Default)]
pub struct PuzParser {
    strict: bool,
}

pub(crate) struct PuzState<'a> {
    warnings: Vec<Error>,

    input: &'a [u8],
    pos: usize,
}

#[derive(Default)]
pub struct TxtParser;

pub(crate) struct TxtState<'a> {
    input: &'a str,
    lines: Lines<'a>,

    pos: usize,
    len: Option<usize>,
}

impl<'a> TxtParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&self, input: &'a str) -> Result<Puzzle> {
        let mut state = TxtState {
            input,
            lines: input.lines(),
            pos: 0,
            len: None,
        };

        let squares = self.parse_grid(&mut state)?;
        let mut puzzle = Puzzle::from_squares(squares);

        let clues = self.parse_clues(&mut state)?;
        puzzle.insert_clues(clues);

        puzzle = self.parse_strings(puzzle, &mut state)?;

        Ok(puzzle)
    }

    #[cfg(feature = "miette")]
    fn report<T>(&self, result: Result<T>, input: &str) -> miette::Result<T> {
        result.map_err(|err| {
            use miette::{NamedSource, Report};

            let source = NamedSource::new(".txt", input.to_string());
            Report::new(err).with_source_code(source)
        })
    }
}

pub(crate) type Span = Range<usize>;

impl<'a> PuzParser {
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    pub fn parse(&self, input: &'a [u8]) -> Result<Puzzle> {
        let (puzzle, _) = self.parse_with_warnings(input)?;
        Ok(puzzle)
    }

    pub fn parse_with_warnings(&self, input: &'a [u8]) -> Result<(Puzzle, Vec<Warning>)> {
        // Parse and validate the main contents of the puzzle
        let mut state = PuzState {
            input,
            pos: 0,
            warnings: vec![],
        };

        let header = self.parse_header(&mut state)?;

        let ((grid, strings), file_span) = state.read_span(|s| {
            let grid = self.parse_grid(header.width, header.height, s)?;
            let strings = self.parse_strings(header.clue_count as usize, s)?;

            Ok((grid, strings))
        })?;

        self.validate_file_checksum(&file_span, &header, &grid, &strings, &mut state)?;
        self.validate_masked_checksums(&header, &grid, &strings, &mut state)?;

        // Parse extra sections and the actual structure of the puzzle
        let extras = self.parse_extras(header.width, header.height, &mut state)?;

        let squares = self.read_squares(&grid, &extras)?;
        let clues = self.read_clues(&squares, &strings)?;

        // Build the puzzle with owned data
        let mut puzzle = Puzzle::new(squares, clues)
            .with_author(PuzState::build_string(strings.author))
            .with_copyright(PuzState::build_string(strings.copyright))
            .with_notes(PuzState::build_string(strings.notes))
            .with_title(PuzState::build_string(strings.title))
            .with_version(PuzState::build_string(header.version));

        if let Some(timer) = &extras.ltim {
            *puzzle.timer_mut() = *timer;
        }

        Ok((puzzle, state.warnings.clone()))
    }

    pub(crate) fn ok_or_warn<T>(
        &self,
        result: Result<T>,
        state: &mut PuzState,
    ) -> Result<Option<T>> {
        match result {
            // Pass through ok/err with strict mode normally
            Ok(val) => Ok(Some(val)),
            Err(err) if self.strict => Err(err),

            // Warn against errors in non-strict mode
            Err(warning) => {
                state.warnings.push(warning);

                Ok(None)
            }
        }
    }

    #[cfg(feature = "miette")]
    pub fn report<T>(&self, result: Result<T>, input: &[u8]) -> miette::Result<T> {
        result.map_err(|err| {
            use miette::{NamedSource, Report};

            let source = NamedSource::new(".puz", PuzState::build_string(input));
            Report::new(err).with_source_code(source)
        })
    }
}

impl<'a> PuzState<'a> {
    pub(crate) fn read_span<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<(T, Span)> {
        // Keep track of where the region starts and read the region
        let start = self.pos;
        let value = f(self)?;

        // Then get the end and span from the current position
        let end = self.pos;
        let span = start..end;

        Ok((value, span))
    }
}

#[cfg(test)]
mod tests {
    use crate::Puzzle;
    use crate::parse::{Error, PuzParser, TxtParser};
    use rstest::rstest;
    use std::{fs, path::PathBuf};

    #[cfg(feature = "miette")]
    type ParseResult<T> = miette::Result<T>;

    #[cfg(not(feature = "miette"))]
    type ParseResult<T> = Result<T, Error>;

    fn parse_puz(path: PathBuf, strict: bool) -> ParseResult<(Puzzle, Vec<Error>)> {
        let bytes = fs::read(&path).expect("puzzle file exists");
        let parser = PuzParser::new(strict);
        let result = parser.parse_with_warnings(&bytes);

        #[cfg(feature = "miette")]
        let result = parser.report(result, &bytes);

        result
    }

    fn parse_txt(path: PathBuf) -> ParseResult<Puzzle> {
        let text = fs::read_to_string(&path).expect("puzzle file exists");
        let parser = TxtParser::new();
        let result = parser.parse(text.as_str());

        #[cfg(feature = "miette")]
        let result = parser.report(result, &text);

        result
    }

    #[rstest]
    fn parse_ok_puz(#[files("puzzles/ok/*.puz")] path: PathBuf) {
        let result = parse_puz(path, false);
        let (puzzle, _) = result.expect("puzzle is parsed correctly");

        assert!(puzzle.rows() > 0);
        assert!(puzzle.cols() > 0);
    }

    #[rstest]
    fn parse_ok_txt(#[files("puzzles/ok/*.txt")] path: PathBuf) {
        let result = parse_txt(path.clone());
        let puzzle = result.expect("puzzled is parsed correctly");

        assert!(puzzle.rows() > 0);
        assert!(puzzle.cols() > 0);
    }

    #[rstest]
    fn parse_err_puz(#[files("puzzles/err/*.puz")] path: PathBuf) {
        let result = parse_puz(path, true);
        let err = result.expect_err("puzzle is not parsed correctly");

        eprintln!("{err}");
    }

    #[rstest]
    fn parse_warn(#[files("puzzles/warn/*.puz")] path: PathBuf) {
        let result = parse_puz(path, false);
        let (_, warnings) = result.expect("puzzle is parsed correctly");

        assert!(!warnings.is_empty());
    }

    #[rstest]
    fn parse_same(#[files("puzzles/**/*.puz")] path: PathBuf) {
        let result = parse_puz(path.clone(), false);

        let txt_path = path.with_extension("txt");

        // Make sure .puz and .txt files parse the same if both defined
        if txt_path.exists() {
            let result2 = parse_txt(txt_path);

            match result {
                Ok((puzzle, _)) => {
                    assert!(result2.is_ok_and(|puzzle2| puzzle == puzzle2));
                }
                Err(_) => {
                    assert!(result2.is_err());
                }
            }
        }
    }
}
