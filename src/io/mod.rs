//! Defines all functionality for reading and writing [puzzles](crate::Puzzle)
//!
//! Deserializing and parsing [*.puz files][PUZ google spec]
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

pub(crate) mod read;
pub(crate) mod write;

mod checksums;
mod error;
mod extras;
mod format;
mod grids;
mod header;
mod strings;

pub use {
    read::{Error as ReadError, PuzRead, PuzReader, Result as ReadResult, TxtReader, Warning},
    write::{PuzWrite, PuzWriter},
};

pub(crate) use {
    checksums::*,
    error::*,
    extras::*,
    grids::*,
    header::*,
    read::{TxtState, build_string, windows_1252_to_char},
    strings::*,
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
