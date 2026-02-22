//! Defines TODO: write
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
//! # Validating checksums
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
//! ```no_run
//! use puzzled::puz::{find_region_checksum, Header};
//!
//! fn validate_cib_checksum<'a>(header: &Header) {
//!     let cib_checksum = find_region_checksum(&header.cib_region, 0);
//!     assert_eq!(cib_checksum, header.cib_checksum);
//! }
//! ```
//!
//! ### File
//! Next is the **file checksum** checks all data used for the [puzzle](crate::Crossword), i.e. both [puzzle grids](self#puzzle-grid) and all [strings](self#strings).
//! Below is a rough outline of how it is validated in the parser
//! ```no_run
//! use puzzled::puz::{find_region_checksum, Header, Grids, Strings};
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
//! use puzzled::puz::{find_region_checksum, find_strings_checksum, Header, Grids, Strings};
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

pub mod read;
pub mod write;

use puzzled_core::format;
pub use read::{PuzRead, PuzReader, Span, build_string, windows_1252_to_char};
pub(crate) use read::{PuzState, Warning};
pub use write::{PuzWrite, PuzWriter};

mod checksums;
mod extras;
mod grids;
mod header;
mod size;
mod strings;

pub use checksums::*;
pub use extras::*;
pub use grids::*;
pub use header::*;
pub use size::*;
pub use strings::*;

pub trait Puz: Sized {
    // Read from puzzle
    fn to_header(&self) -> format::Result<Header>;
    fn to_grids(&self) -> format::Result<Grids>;
    fn to_strings(&self) -> format::Result<Strings>;
    fn to_extras(&self) -> format::Result<Extras>;

    // Write as puzzle
    fn from_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<Self>;
}

pub trait Context<T, E> {
    fn context<S: Into<String>>(self, context: S) -> std::result::Result<T, E>;
}

impl<T> Context<T, read::Error> for format::Result<T> {
    fn context<S: Into<String>>(self, context: S) -> read::Result<T> {
        self.map_err(|err| read::Error {
            kind: read::ErrorKind::Format(err),
            span: Span::default(),
            context: context.into(),
        })
    }
}

impl<T> Context<T, write::Error> for format::Result<T> {
    fn context<S: Into<String>>(self, context: S) -> write::Result<T> {
        self.map_err(|err| write::Error {
            kind: write::ErrorKind::Format(err),
            context: context.into(),
        })
    }
}
