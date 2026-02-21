use crate::{Context, PuzRead, PuzState, PuzWrite, Version, read, write};

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Header {
    // Components
    pub version: Option<Version>,
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
        let version = reader.read_slice::<4>().context("Version bytes")?;

        let version = state.ok_or_warn(Version::new(&version).context("Version"))?;

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

        let version = &self.version.unwrap_or_default().as_bytes();
        writer.write_all(version).context("Version")?;

        writer.write_all(&self.reserved).context("Revealed1C")?;
        writer
            .write_u16(self.scrambled_checksum)
            .context("Scrambled checksum")?;

        writer.write_all(&self.cib_region).context("CIB region")?;
        Ok(())
    }
}
