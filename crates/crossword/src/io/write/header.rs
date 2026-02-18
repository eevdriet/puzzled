use crate::io::{Context, Header, PuzWrite, write};

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
