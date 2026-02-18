use crate::{
    Crossword, SizeCheck,
    io::{format, is_valid_version},
};

#[derive(Debug, Default)]
pub(crate) struct Header {
    // Components
    pub version: [u8; 4],
    pub width: u8,
    pub height: u8,
    pub clue_count: u16,

    // Checksums
    pub file_checksum: u16,
    pub cib_checksum: u16,
    pub mask_checksums: [u8; 8],
    pub scrambled_checksum: u16,

    // Regions
    pub cib_region: [u8; 8],

    // Other
    pub file_magic: [u8; 12],
    pub reserved: [u8; 14],
    pub scrambled_tag: u16,
}

impl Header {
    pub(crate) fn from_puzzle(puzzle: &Crossword) -> format::Result<Self> {
        puzzle.squares().check_size()?;

        let mut header = Header::default();

        if let Some(version) = puzzle.version() {
            let bytes = version.as_bytes();
            if is_valid_version(bytes) {
                return Err(format::Error::InvalidVersion);
            }

            header.version = [bytes[0], bytes[1], bytes[2], bytes[3]];
        }

        // Fill the CIB region
        header.width = puzzle.cols() as u8;
        header.height = puzzle.rows() as u8;
        header.clue_count = puzzle.clues().len() as u16;
        header.write_cib();

        Ok(header)
    }

    pub(crate) fn read_cib(&mut self) {
        let cib = self.cib_region;

        self.width = cib[0];
        self.height = cib[1];
        self.clue_count = u16::from_le_bytes([cib[2], cib[3]]);
        self.scrambled_tag = u16::from_le_bytes([cib[6], cib[7]]);
    }

    pub(crate) fn write_cib(&mut self) {
        self.cib_region[0] = self.width;
        self.cib_region[1] = self.height;
        self.cib_region
            .copy_from_slice(&self.clue_count.to_le_bytes());
    }
}
