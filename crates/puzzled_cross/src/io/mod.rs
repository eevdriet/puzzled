//! Defines all functionality for reading and writing [puzzles](crate::Crossword)
//!
//! # Formats
//! The crate currently supports the following formats and streams for dealing with them:
//! | Format  | Reader | Writer |
//! |------------|--------|------|
//! | Binary | [`PuzReader`] | [`PuzWriter`] |
//! | Text | [`TxtReader`] | |
//!
//! ## Binary
//! This crate tries to following the [Across Lite format][PUZ google spec] as closely as possible to handle binary data.
//! Note that a [reformatted specification][PUZ spec] which may be easier to read from, as it provides the full specification in Markdown.
//! The format is used to create `*.puz` files, which are commonly shared online on platforms such as [Crosshare](https://crosshare.org/).
//!
//! ## Text
//! The **text** format allows for a more WYSIWYG definition of puzzles.
//! It ties in nicely with the [`crossword!`](crate::crossword!) macro, as its [DSL](https://doc.rust-lang.org/rust-by-example/macros/dsl.html) follows the text format exactly.
//!
//! For example, the following two ways to construct a puzzle are identical
//! ```
//! use puzzled_crossword::{crossword};
//! use puzzled_crossword::io::{TxtReader};
//! use std::{path::Path, fs::read_to_string};
//!
//! // 1. Macro definition
//! let puzzle1 = crossword! {
//!     [A B]
//!     [C .]
//!     - A: "The first two letters of the alphabet"
//!     - D: "Keep it short, but cool"
//!     version: "2.0"
//! };
//!
//! // 2. Text file
//! let path = Path::new("puzzles/ok/alphabet.txt");
//! let txt = read_to_string(path)?;
//!
//! let reader = TxtReader;
//! let puzzle2 = reader.read(&txt)?;
//!
//! assert_eq!(puzzle1, puzzle2);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Errors and warnings
//! The crate defines 3 types of [`Error`] that can occur when reading and writing [puzzles](crate::Crossword):
//! - [`read::Error`] for errors that occur when *reading puzzles*.
//!
//!   For example, [`PuzReader`] may not read the required number of [clues](crate::Clues) and yield a missing clue error.
//! - [`write::Error`] for errors that occur when *writing puzzles*
//!
//!   For example, [`PuzWriter`] fails writing a large puzzle to a small output buffer
//! - [`format::Error`] for errors can occur during both *reading and writing puzzles*.
//!
//!   For example, a `*.puz` file requires that its version is set as `x.y` where `x,y` are one digit numbers.
//!   The [`PuzReader`] may read a series of bytes that doesn't agree with that format.
//!   Similary, the [`PuzWriter`] may not be able to convert the [`Crossword::version`](crate::Crossword::version) when writing out the byte data.
//!   Both scenarios would yield a [`InvalidVersion`](format::Error::InvalidVersion) error.
//!
//! In some cases, errors are recoverable such that they do not have to impede the whole reading/writing process.
//! For example, when a [`PuzReader`] encounters an invalid [extra section](self::extra-sections), it should be able to just skip it and create a puzzle anyways.
//! Streams that support warnings are initialized with a `strict` flag to indicate how to handle warnings.
//! - If `strict == true`, a warning is treated as an error and the streaming is immediately aborted if one is encountered
//! - Otherwise, all warnings are collected throughout the streaming process.
//!   Streams specify a separate `*_with_warnings` to return them to the user along with the streaming result.
//!   For example, consider [`PuzReader::read`] and [`PuzReader::read_with_warnings`]
//!
//! # Process
//! Even though the [`PuzReader`] and [`PuzWriter`] are the only streams using the [Across Lite format][PUZ google spec] format directly, all streams addhere to its underlying data model.
//! Therefore it may be worthwhile to read through this section, even if you have no interest in working with binary data to represent your [puzzles](crate::Crossword).
//!
//! ## Header
//! First a **[header](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#header)** is read, which mostly contains [checksums][Checksums] to verify whether the binary data is valid.
//! It also defines a [version](crate::Crossword::version) and the basic layout for the puzzle, such as its [width](crate::Grid<Square>::cols), [height](crate::Grid<Square>::rows) and how many [clues](crate::Clues) should be read.
//! We list the components that are read as follows:
//! - <span style="color:white">White</span> components are directly used to define the resulting [puzzle][Crossword].
//! - <span style="color:yellow">Yellow</span> components are used for [validating checksums](self#validating-checksums) and checking the byte integrity of the `*.puz` data.
//! - <span style="color:gray">Gray</span> components are currently ignored
//!
//! | Component  | Length | Type | Description |
//! |------------|--------|------|-------------|
//! | <span style="color:yellow">Checksum</span>   | 2      | u16  | Overall [file checksum](self#file) |
//! | <span style="color:yellow">File Magic</span> | 12     | str  | NUL-terminated constant string: `b"ACROSS&DOWN\0"` |
//! | <span style="color:yellow">CIB Checksum</span>          | 2      | u16  | [CIB checksum](self#cib) |
//! | <span style="color:yellow">Masked Low Checksums</span>  | 4      | u32  | A set of low [masked checksums](self#masked-regions) |
//! | <span style="color:yellow">Masked High Checksums</span> | 4      | u32  | A set of high [masked checksums](self#masked-regions) |
//! | <span style="color:white">Version String(?)</span> | 4      | str  | e.g. "1.2\0" |
//! | <span style="color:gray">Reserved1C(?)</span>      | 2      | u16  | In many files, this is uninitialized memory |
//! | <span style="color:gray">Scrambled Checksum</span> | 2      | u16  | In scrambled puzzles, a checksum of the real solution (details below) |
//! | <span style="color:white">Width</span>        | 1      | u8   | The width of the board |
//! | <span style="color:white">Height</span>             | 1      | u8   | The height of the board |
//! | <span style="color:white"># of Clues</span>  | 2      | u16  | The number of clues for this board |
//! | <span style="color:gray">Unknown Bitmask</span>    | 2      | u16  | A bitmask. Operations unknown. |
//! | <span style="color:gray">Scrambled Tag</span>      | 2      | u16  | 0 for unscrambled puzzles. Nonzero (often 4) for scrambled puzzles. |
//!
//! ## Crossword grids
//! Then the **[grids](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#puzzle-layout-and-state)** which define the layout of [puzzle][Crossword] are read.
//! Specifically, the following 2 grids are read from the `header.width` and `header.height`:
//! 1.  A *solution* grid containing the [solution](crate::Solution) to each [square](crate::Square)
//!     To indicate a [non-playable (black) square](crate::Square::Black), a `b"."` is used.
//!     The other squares are the playable [cells](crate::Cell) that the user can put their solutions into.
//! 2.  A *state* grid containing the current [entry](crate::Cell::entry) to each square
//!     Note that *even if a user has not yet entered any solutions, a full state grid is read*.
//!     Cells that do not yet contain a user entry are indicate with `b"-"`
//!
//! As an example, consider the following puzzle and its underlying puzzle grids in binary form:
//! ```
//! use puzzled_crossword::crossword;
//!
//! let puzzle = crossword! (
//!     [C . .]
//!     [A . .]
//!     [R O W]
//! );
//!
//! // Underlying byte data to represent the puzzle grids
//! // Note that the `crossword!` macro doesn't include user entries
//! let solution = b"C..A..ROW";
//! let state = b"-..-..---";
//! ```
//!
//! The crate uses a [`Grid<Square>`](crate::Grid<Square>) to store both the solution and state in a single grid.
//!
//! ## Strings
//! Next are the **[strings](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#strings-section)**, which include the "metadata" for the [puzzle](crate::Crossword), such as its [title](crate::Crossword::title) and [author](crate::Crossword::author).
//! It also includes the text for every [clue](crate::Clue) that should be placed in the puzzle [grid](crate::Squares).
//! The pseudo-code of how to do so is given [here](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#clue-assignment) and the crate implementation is found in [`Crossword::place_clues`](crate::Crossword::place_clues).
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
//!
//! ## Extra sections
//! Finally a number of **[extra sections][PUZ google spec]** are optionally read to allow for reading square customization and [solving time](crate::Timer).
//! The crate currently supports **GRBS**, **RTBL**, **LTIM** and **GEXT** sections are considered, but more may be supported in the future.
//!
//! ### GRBS and RTBL
//! The **GRBS** section contains a [grid](crate::Grid) of keys for each [square](crate::Square) in the [puzzle](crate::Crossword) that has a [rebus solution](crate::Solution::Rebus).
//! The actual rebus values themselves are read afterwards in the **RTBL** section.
//! For a [non-rebus (letter) square](crate::Solution::Letter), a `0` byte is used to indicate no rebus needs to be read in RTBL.
//! Any square that *does* contain a rebus gets a unique identifying byte key of `n`.
//!
//! The **RTBL** section then contains an ASCII-string representing the actual rebuses.
//! It is read as a [`HashMap<u8, String>`](std::collections::HashMap) and correctly sets a rebus solution for the squares represented in GRBS.
//! Consider the following example to get an idea of how the GRBS and RTBL sections would be layed out in a `*.puz` file:
//! ```
//! use puzzled_crossword::crossword;
//!
//! let puzzle = crossword! (
//!     [C      REBUS1 Y     ]
//!     [A      .      REBUS1]
//!     [REBUS2 O      W     ]
//! );
//!
//! // Binary data read in the GRBS and RTBL extras section to represent the puzzle above
//! // Note that
//! // - The keys are not necessarily consecutive numbers
//! // - The same key can be used multiple times in GRBS (e.g. `7`)
//! // - Keys are always represented with 2 digits, so for 1-9 a leading space is used (e.g. ` 7`)
//! let grbs = [0, 7, 0, 0, 0, 7, 16, 0, 0];
//! let rtbl = b" 7:REBUS1;16:REBUS2";
//! ```
//!
//! ### LTIM
//! The **LTIM** section contains the definition of a [`Timer`](crate::Timer) which represents the time already used solving the [puzzle](crate::Crossword).
//! Specifically, the following are read
//! -   A [`Duration`](std::time::Duration) from a *number of elapsed seconds*
//! -   A [`TimerState`](crate::TimerState) representing whether the timer is active (`0` for [`TimerState::Running`](crate::TimerState::Running) and `1` for [`TimerState::Stopped`](crate::TimerState::Stopped))
//!
//! ### GEXT
//! The **GEXT** sections contains a [grid](crate::Grid) of [styles](crate::CellStyle) that are applied to each of the [squares](crate::Square) in the [puzzle](crate::Crossword).
//! Each style is represnted with a single [`u8`], where [non-playable squares](crate::Square::Black) always take the value `0`.
//! For a [cell](crate::Cell), refer to [`CellStyle`](crate::CellStyle) to see which styles are currently supported.
//! Multiple styles can be set at once as style is represented as (partially complete) bit flags.
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
//! When `strict` reading is enabled, all checksums need to be valid in order to successfully parse a [puzzle](crate::Crossword).
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
//! Next is the **file checksum** checks all data used for the [puzzle](crate::Crossword), i.e. both [puzzle grids](self#puzzle-grid) and all [strings](self#strings).
//! Below is a rough outline of how it is validated in the parser
//! ```no_run
//! use puzzled_crossword::io::{find_region_checksum, Header, Grids, Strings};
//!
//! fn find_strings_checksum(strings: &Strings, start: u16) -> u16 {
//!     let mut checksum = start;
//!
//!     if strings.title.len() > 1 {
//!         checksum = find_region_checksum(&strings.title, checksum);
//!     }
//!     if strings.author.len() > 1 {
//!         checksum = find_region_checksum(&strings.author, checksum);
//!     }
//!     if strings.copyright.len() > 1 {
//!         checksum = find_region_checksum(&strings.copyright, checksum);
//!     }
//!
//!     for clue in &strings.clues {
//!         checksum = find_region_checksum(&clue[..clue.len() - 1], checksum);
//!     }
//!     
//!     if strings.notes.len() > 1 {
//!         checksum = find_region_checksum(&strings.notes, checksum);
//!     }
//!
//!     checksum
//! }
//!
//! fn validate_file_checksum<'a>(
//!     header: &Header,
//!     grids: &Grids,
//!     strings: &Strings,
//! ) {
//!     // Compute the overall file checksum
//!     let mut file_checksum = header.cib_checksum;
//!
//!     file_checksum = find_region_checksum(grids.solution.data(), file_checksum);
//!     file_checksum = find_region_checksum(grids.state.data(), file_checksum);
//!     file_checksum = find_strings_checksum(strings, file_checksum);
//!
//!     assert_eq!(file_checksum, header.file_checksum);
//! }
//!
//! ```
//!
//! ### Masked regions
//! Finally we need to validate the [masked regions](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#masked-checksums) given in the [header](self#header).
//! It comes down the calculating the checksums we've seen before and validating them with the `header.mask_checksums`.
//! To do so, amusingly we use the phrase `I CHEATED` and then some XOR-ing and bit shifting.
//!
//! Below is a rough outline of how it is validated in the parser
//! ```no_run
//! use puzzled_crossword::io::{find_region_checksum, find_strings_checksum, Header, Grids, Strings};
//!
//! fn validate_masked_checksums<'a>(
//!     header: &Header,
//!     grids: &Grids,
//!     strings: &Strings,
//! ) {
//!     let cib_checksum = find_region_checksum(&header.cib_region, 0);
//!     let sol_checksum = find_region_checksum(grids.solution.data(), 0);
//!     let state_checksum = find_region_checksum(grids.state.data(), 0);
//!     let strs_checksum = find_strings_checksum(strings, 0);
//!
//!     assert_eq!(header.mask_checksums[0], b'I' ^ (cib_checksum & 0xFF) as u8);
//!     assert_eq!(header.mask_checksums[1], b'C' ^ (sol_checksum & 0xFF) as u8);
//!     assert_eq!(header.mask_checksums[2], b'H' ^ (state_checksum & 0xFF) as u8);
//!     assert_eq!(header.mask_checksums[3], b'E' ^ (strs_checksum & 0xFF) as u8);
//!     assert_eq!(header.mask_checksums[4], b'A' ^ ((cib_checksum & 0xFF00) >> 8) as u8);
//!     assert_eq!(header.mask_checksums[5], b'T' ^ ((sol_checksum & 0xFF00) >> 8) as u8);
//!     assert_eq!(header.mask_checksums[6], b'E' ^ ((state_checksum & 0xFF00) >> 8) as u8);
//!     assert_eq!(header.mask_checksums[7], b'D' ^ ((strs_checksum & 0xFF00) >> 8) as u8);
//! }
//! ```
//! [puzzled]: crate
//! [Crossword]: crate::Crossword
//! [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki
//! [Checksums]: self#validating-checksums

pub(crate) mod format;
pub(crate) mod read;
pub(crate) mod write;

mod checksums;
mod error;
mod extras;
mod grids;
mod header;
mod strings;

pub use {checksums::*, grids::Grids, header::Header, strings::Strings};
pub use {
    error::*,
    format::{Error as FormatError, Result as FormatResult},
    read::{Error as ReadError, PuzRead, PuzReader, Result as ReadResult, TxtReader, Warning},
    write::{Error as WriteError, PuzWrite, PuzWriter, Result as WriteResult},
};

pub(crate) use {
    extras::*,
    grids::*,
    read::{TxtState, build_string, windows_1252_to_char},
};

pub(crate) const SECTION_SEPARATOR: &str = "---";
pub(crate) const FILE_MAGIC: &str = "ACROSS&DOWN\0";

use std::ops::Range;

pub(crate) type Span = Range<usize>;

pub(crate) fn is_valid_version(version: &[u8]) -> bool {
    // Optionally strip the trailing \0
    let version = version.strip_suffix(&[0]).unwrap_or(version);

    if version.len() != 3 {
        return false;
    }

    let mut bytes = version.iter();
    let (x, &dot, y) = (
        bytes.next().expect("checked version length"),
        bytes.next().expect("checked version length"),
        bytes.next().expect("checked version length"),
    );

    x.is_ascii_digit() && dot == b'.' && y.is_ascii_digit()
}

pub(crate) trait SizeCheck {
    fn check_size(&self) -> format::Result<()>;
}
