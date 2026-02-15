use thiserror::Error;

use crate::parse::{
    Error, PuzParser, PuzState, Result, Span, checksums::find_region_checksum, parse_string,
};

const FILE_MAGIC: &str = "ACROSS&DOWN\0";

#[derive(Debug)]
pub(crate) struct Header<'a> {
    // Components
    pub version: &'a [u8],
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
    pub cib_span: Span,
    pub masks_span: Span,
}

#[derive(Debug, Error, Clone)]
pub enum HeaderError {
    #[error("Invalid file magic: .puz files expect '{FILE_MAGIC}', but found '{found}'")]
    InvalidFileMagic { found: String },

    #[error("Invalid puzzle dimensions: read {width} width and {height} height")]
    InvalidDimensions { width: u8, height: u8 },
}

impl<'a> PuzParser {
    pub(crate) fn parse_header(&self, state: &mut PuzState<'a>) -> Result<Header<'a>> {
        let file_checksum = state.read_u16("Checksum")?;

        // Read the file magic that validates a .puz file
        let (magic, magic_span) = state.read_span(|p| p.read(FILE_MAGIC.len(), "File Magic"))?;

        if magic != FILE_MAGIC.as_bytes() {
            return Err(Error {
                span: magic_span,
                kind: HeaderError::InvalidFileMagic {
                    found: parse_string(magic),
                }
                .into(),
                context: "File magic".to_string(),
            });
        }
        let cib_checksum = state.read_u16("CIB Checksum")?;
        let ((low_checksums, high_checksums), masks_span) = state.read_span(|p| {
            let low = p.read(4, "Masked Low Checksums")?;
            let high = p.read(4, "Masked High Checksums")?;

            Ok((low, high))
        })?;
        let version = state.read_fixed_len_str(4, "Version")?;

        // self.skip(2, "Reserved1C")?;
        state.skip(16, "Unimportant")?;

        // let scrambled_checksum = self.read_u16("Scrambled Checksum")?;
        let scrambled_checksum = 0;

        // Validate the CIB checksum
        let cib_region = state.take(8, "CIB checksum region")?;
        let cib_span = state.pos..state.pos + 8;
        let cib_region_checksum = find_region_checksum(cib_region, 0);

        self.validate_checksum(
            cib_span.clone(),
            "CIB".to_string(),
            cib_checksum,
            cib_region_checksum,
            state,
        )?;

        let ((width, height), dim_span) = state.read_span(|p| {
            let width = p.read_u8("Puzzle width")?;
            let height = p.read_u8("Puzzle height")?;

            Ok((width, height))
        })?;

        if width == 0 || height == 0 {
            return Err(Error {
                span: dim_span,
                context: "Puzzle dimensions".to_string(),
                kind: HeaderError::InvalidDimensions { width, height }.into(),
            });
        }

        let clue_count = state.read_u16("Clue count")?;

        state.skip(2, "Unknown Bitmask")?;

        let scrambled_tag = state.read_u16("Scrambled Tag")?;

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

            // Spans
            cib_span,
            masks_span,
        })
    }
}
