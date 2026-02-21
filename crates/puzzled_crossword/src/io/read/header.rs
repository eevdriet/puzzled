use crate::io::{Context, Header, PuzRead, read};

impl Header {
    pub(crate) fn read_from<R: PuzRead>(reader: &mut R) -> read::Result<Self> {
        let file_checksum = reader.read_u16().context("File checksum")?;
        let file_magic = reader.read_slice::<12>().context("File magic")?;

        let cib_checksum = reader.read_u16().context("CIB checksum")?;
        let mask_checksums = reader.read_slice::<8>().context("Masked checksums")?;
        let version = reader.read_slice::<4>().context("Version")?;

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
