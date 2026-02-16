use std::io::{Seek, SeekFrom, Write};

use crate::io::{
    PuzWriter, Strings, find_file_checksum, find_mask_checksums, find_region_checksum,
    find_strings_checksum,
    write::{Grids, Header, PuzWrite},
};

impl PuzWriter {
    pub(crate) fn write_checksums(
        &self,
        header: &mut Header,
        grids: &Grids,
        strings: &Strings,
    ) -> std::io::Result<()> {
        let cursor = &mut header.cursor;

        // Find and write the file checksum
        let cib_checksum = header.cib_checksum;
        let file_checksum =
            find_file_checksum(cib_checksum, &grids.solution, &grids.state, strings);

        cursor.seek(SeekFrom::Start(header.file_pos))?;
        cursor.write_u16(file_checksum)?;

        // Find and write the masks checksum
        let solution_checksum = find_region_checksum(&grids.solution, 0);
        let state_checksum = find_region_checksum(&grids.state, 0);
        let strings_checksum = find_strings_checksum(&strings, 0);

        let mask_checksums = find_mask_checksums(
            cib_checksum,
            solution_checksum,
            state_checksum,
            strings_checksum,
        );

        cursor.seek(SeekFrom::Start(header.masks_pos))?;
        cursor.write_all(&mask_checksums)?;

        // Finish
        cursor.seek(SeekFrom::End(0))?;
        Ok(())
    }
}
