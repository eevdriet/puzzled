use std::borrow::Cow;

use thiserror::Error;

use crate::{Parser, Region, Result};

const FILE_MAGIC: &[u8] = b"ACROSS&DOWN\0";

#[derive(Debug)]
pub(crate) struct Header<'a> {
    // Components
    pub version: Cow<'a, str>,
    pub width: u8,
    pub height: u8,
    pub clue_count: u16,
    pub scrambled_tag: u16,

    // Checksums
    pub file_checksum: u16,
    pub cib_checksum: u16,
    pub low_checksums: &'a [u8],
    pub high_checksums: &'a [u8],
    pub scrambled_checksum: u16,

    // Regions
    pub cib_region: Region<'a>,
}

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("Invalid file magic: .puz files expect '{FILE_MAGIC:?}', but found '{found:?}'")]
    InvalidFileMagic { found: Vec<u8>, pos: usize },
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_header(&mut self) -> Result<Header<'a>> {
        // | Component  | Length | Type | Description |
        // |------------|--------|------|-------------|
        // | Checksum   | 2      | u16  | overall file checksum |
        // | File Magic | 12     | str  | NUL-terminated constant string: `4143 524f 5353 2644 4f57 4e00` (`"ACROSS&DOWN"`) |
        let file_checksum = self.read_u16("Checksum")?;

        // Read the file magic that validates a .puz file
        let magic = self.read(FILE_MAGIC.len(), "File Magic")?;

        if magic != FILE_MAGIC {
            return Err(HeaderError::InvalidFileMagic {
                found: magic.to_vec(),
                pos: self.pos,
            }
            .into());
        }

        // CIB and masked low/high checksums
        // | Component             | Length | Type | Description |
        // |-----------------------|--------|------|-------------|
        // | CIB Checksum          | 2      | u16  | (defined later) |
        // | Masked Low Checksums  | 4      | u32  | A set of checksums, XOR-masked against a magic string. |
        // | Masked High Checksums | 4      | u32  | A set of checksums, XOR-masked against a magic string. |
        let cib_checksum = self.read_u16("CIB Checksum")?;
        let low_checksums = self.read(4, "Masked Low Checksums")?;
        let high_checksums = self.read(4, "Masked High Checksums")?;

        // | Component          | Length | Type | Description |
        // |--------------------|--------|------|-------------|
        // | Version String(?)  | 4      | str  | e.g. "1.2\0" |
        // | Reserved1C(?)      | 2      | u16  | In many files, this is uninitialized memory |
        // | Scrambled Checksum | 2      | u16  | In scrambled puzzles, a checksum of the real solution (details below) |
        // | Width              | 1      | u8   | The width of the board |
        // | Height             | 1      | u8   | The height of the board |
        // | # of Clues         | 2      | u16  | The number of clues for this board |
        // | Unknown Bitmask    | 2      | u16  | A bitmask. Operations unknown. |
        // | Scrambled Tag      | 2      | u16  | 0 for unscrambled puzzles. Nonzero (often 4) for scrambled puzzles. |
        let version = self.read_fixed_len_str(4, "Version")?;

        self.skip(2, "Reserved1C")?;

        let scrambled_checksum = self.read_u16("Scrambled Checksum")?;

        let ((width, height, clue_count, scrambled_tag), cib_region) = self.read_region(|p| {
            let width = p.read_u8("Puzzle width")?;
            let height = p.read_u8("Puzzle height")?;
            let clue_count = p.read_u16("Clue count")?;

            p.skip(2, "Unknown Bitmask")?;

            let scrambled_tag = p.read_u16("Scrambled Tag")?;

            Ok((width, height, clue_count, scrambled_tag))
        })?;

        Ok(Header {
            // Components
            version,
            width,
            height,
            clue_count,
            scrambled_tag,

            // Checksums
            file_checksum,
            cib_checksum,
            low_checksums,
            high_checksums,
            scrambled_checksum,

            // Regions
            cib_region,
        })
    }
}
