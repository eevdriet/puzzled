use std::io::{Cursor, Seek, SeekFrom, Write};

use crate::{
    Puzzle,
    io::{FILE_MAGIC, PuzWriter, find_cib_checksum, write::PuzWrite},
};

#[derive(Debug)]
pub(crate) struct Header {
    pub cursor: Cursor<Vec<u8>>,

    pub file_pos: u64,
    pub masks_pos: u64,

    pub cib_checksum: u16,
}

impl PuzWriter {
    pub(crate) fn write_header(&self, puzzle: &Puzzle) -> std::io::Result<Header> {
        let mut header = Cursor::new(Vec::new());

        // File checksum
        let file_pos = header.position();
        header.pad(2)?;

        // File magic
        header.write_all(FILE_MAGIC.as_bytes())?;

        // CIB checksum
        let cib_checksum_pos = header.position();
        header.pad(2)?;

        // Masked checksums
        let masks_pos = header.position();
        header.pad(8)?;

        // Version
        let version = puzzle.version();
        assert!(version.is_none_or(|v| v.len() == 3));

        header.write_opt_str0(version, 3)?;

        // Unimportant
        header.pad(16)?;

        // Construct CIB region
        let mut cib = Cursor::new(Vec::new());

        cib.write_u8(puzzle.cols())?; // Width
        cib.write_u8(puzzle.rows())?; // Height
        cib.write_u16(puzzle.clues().len() as u16)?; // Clue count
        cib.pad(4)?; // Unknown bitmask + scrambled

        let cib_region = cib.into_inner();

        // Retroactively write the CIB checksum
        let cib_checksum = find_cib_checksum(&cib_region);

        header.seek(SeekFrom::Start(cib_checksum_pos))?;
        header.write_u16(cib_checksum)?;
        header.seek(SeekFrom::End(0))?;

        Ok(Header {
            cursor: header,
            file_pos,
            masks_pos,
            cib_checksum,
        })
    }
}
