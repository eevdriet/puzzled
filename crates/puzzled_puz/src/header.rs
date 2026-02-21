use crate::{Context, PuzRead, PuzState, PuzWrite, format, read, write};
use puzzled_core::Version;

pub(crate) const FILE_MAGIC: &str = "ACROSS&DOWN\0";

/// [Header](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#header) section
///
/// This section mostly contains [checksums](crate#validating-checksums) to verify whether the binary data is valid.
/// It also defines a [version](puzzled_core::Version) and the basic layout of a [puzzle](crate::Puz), such as its width, height and how many [clues](puzzled::crossword::Clues) should be read.
/// We list the components that are read as follows:
/// - <span style="color:white">White</span> components are directly used to define the resulting [puzzle][Crossword].
/// - <span style="color:yellow">Yellow</span> components are used for [validating checksums](self#validating-checksums) and checking the byte integrity of the `*.puz` data.
/// - <span style="color:gray">Gray</span> components are currently ignored
///
/// | Component  | Length | Type | Description |
/// |------------|--------|------|-------------|
/// | <span style="color:yellow">Checksum</span>   | 2      | u16  | Overall [file checksum](crate#file) |
/// | <span style="color:yellow">File Magic</span> | 12     | str  | NUL-terminated constant string: `b"ACROSS&DOWN\0"` |
/// | <span style="color:yellow">CIB Checksum</span>          | 2      | u16  | [CIB checksum](crate#cib) |
/// | <span style="color:yellow">Masked Low Checksums</span>  | 4      | u32  | A set of low [masked checksums](crate#masked-regions) |
/// | <span style="color:yellow">Masked High Checksums</span> | 4      | u32  | A set of high [masked checksums](crate#masked-regions) |
/// | <span style="color:white">Version String(?)</span> | 4      | str  | e.g. "1.2\0" |
/// | <span style="color:gray">Reserved1C(?)</span>      | 2      | u16  | In many files, this is uninitialized memory |
/// | <span style="color:gray">Scrambled Checksum</span> | 2      | u16  | In scrambled puzzles, a checksum of the real solution (details below) |
/// | <span style="color:white">Width</span>        | 1      | u8   | The width of the board |
/// | <span style="color:white">Height</span>             | 1      | u8   | The height of the board |
/// | <span style="color:white"># of Clues</span>  | 2      | u16  | The number of clues for this board |
/// | <span style="color:gray">Unknown Bitmask</span>    | 2      | u16  | A bitmask. Operations unknown. |
/// | <span style="color:gray">Scrambled Tag</span>      | 2      | u16  | 0 for unscrambled puzzles. Nonzero (often 4) for scrambled puzzles. |
///
#[derive(Debug, Default)]
pub struct Header {
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
    pub fn read_cib(&mut self) {
        let cib = self.cib_region;

        self.width = cib[0];
        self.height = cib[1];
        self.clue_count = u16::from_le_bytes([cib[2], cib[3]]);
        self.scrambled_tag = u16::from_le_bytes([cib[6], cib[7]]);
    }

    pub fn write_cib(&mut self) {
        self.cib_region[0] = self.width;
        self.cib_region[1] = self.height;
        self.cib_region
            .copy_from_slice(&self.clue_count.to_le_bytes());
    }
}

/// # Read
impl Header {
    pub(crate) fn read_from<R: PuzRead>(
        reader: &mut R,
        state: &mut PuzState,
    ) -> read::Result<Self> {
        let file_checksum = reader.read_u16().context("File checksum")?;
        let file_magic = reader.read_slice::<12>().context("File magic")?;

        let cib_checksum = reader.read_u16().context("CIB checksum")?;
        let mask_checksums = reader.read_slice::<8>().context("Masked checksums")?;

        // Try to parse a valid version, otherwise set empty bits
        let version = reader.read_slice::<4>().context("Version bytes")?;
        let version = state.ok_or_warn(
            Version::new(&version)
                .map_err(|reason| format::Error::InvalidVersion { reason })
                .context("Version"),
        )?;
        let version = version.map(|v| v.as_bytes()).unwrap_or_default();

        let reserved = reader.read_slice::<14>().context("Reserved1C")?;
        let scrambled_checksum = reader.read_u16().context("Scrambled checksum")?;

        let cib_region = reader.read_slice::<8>().context("CIB region")?;
        let mut header = Header {
            file_checksum,
            file_magic,
            cib_checksum,
            mask_checksums,
            reserved,
            scrambled_checksum,
            version,
            cib_region,
            ..Default::default()
        };
        header.read_cib();

        Ok(header)
    }
}

/// # Write
impl Header {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        writer
            .write_u16(self.file_checksum)
            .context("File checksum")?;
        writer.write_all(&self.file_magic).context("File magic")?;

        writer
            .write_u16(self.cib_checksum)
            .context("CIB checksum")?;
        writer
            .write_all(&self.mask_checksums)
            .context("Masked checksums")?;

        writer.write_all(&self.version).context("Version")?;

        writer.write_all(&self.reserved).context("Revealed1C")?;
        writer
            .write_u16(self.scrambled_checksum)
            .context("Scrambled checksum")?;

        writer.write_all(&self.cib_region).context("CIB region")?;
        Ok(())
    }
}
