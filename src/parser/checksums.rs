use crate::{Error, Grid, Header, Parser, Region, Result, Strings};

const CHECKSUM_MASK: &[u8; 9] = b"ICHEATED\0";

impl<'a> Parser<'a> {
    pub(crate) fn validate_checksums(
        &mut self,
        header: &Header<'a>,
        grid: &Grid<'a>,
        strings: &Strings<'a>,
    ) -> Result<()> {
        self.validate_cib_checksum(header)
            .and(self.validate_file_checksum(header, grid, strings))
            .and(self.validate_masked_checksums(header, grid, strings))?;

        Ok(())
    }

    fn validate_cib_checksum(&mut self, header: &Header<'a>) -> Result<Option<()>> {
        let cib_checksum = find_region_checksum(&header.cib_region, 0);
        self.validate_checksum("CIB".to_string(), cib_checksum, header.cib_checksum)
    }

    fn validate_file_checksum(
        &mut self,
        header: &Header<'a>,
        grid: &Grid<'a>,
        strings: &Strings<'a>,
    ) -> Result<Option<()>> {
        // Compute the overall file checksum
        let mut file_checksum = header.cib_checksum;

        file_checksum = find_region_checksum(&grid.solution_region, file_checksum);
        file_checksum = find_region_checksum(&grid.state_region, file_checksum);
        file_checksum = find_partial_board_checksum(file_checksum, strings);

        // Validate the file checksum
        self.validate_checksum("file".to_string(), file_checksum, header.file_checksum)
    }

    fn validate_masked_checksums(
        &mut self,
        header: &Header<'a>,
        grid: &Grid<'a>,
        strings: &Strings<'a>,
    ) -> Result<Option<()>> {
        // Collect all checksums to mask
        let checksums = [
            find_region_checksum(&header.cib_region, 0),    // CIB
            find_region_checksum(&grid.solution_region, 0), // Solution
            find_region_checksum(&grid.state_region, 0),    // Grid
            find_partial_board_checksum(0, strings),        // Partial board
        ];

        // Validate against the masked low checksums
        if header.low_checksums.len() != checksums.len() {
            return Err(Error::MissingChecksum {
                kind: "masked low".to_string(),
                found: header.low_checksums.len(),
                expected: checksums.len(),
            });
        }

        for (idx, (&checksum, &expected)) in checksums
            .iter()
            .zip(header.low_checksums.iter())
            .enumerate()
        {
            let kind = format!("Masked low #{}", idx + 1);
            let found = CHECKSUM_MASK[idx] ^ (checksum & 0xFF) as u8;

            self.validate_checksum(kind, found as u16, expected as u16)?;
        }

        // Validate against the masked high checksums
        if header.high_checksums.len() != checksums.len() {
            return Err(Error::MissingChecksum {
                kind: "masked high".to_string(),
                found: header.high_checksums.len(),
                expected: checksums.len(),
            });
        }

        for (idx, (&checksum, &expected)) in checksums
            .iter()
            .zip(header.high_checksums.iter())
            .enumerate()
        {
            let kind = format!("Masked low #{}", idx + 5);
            let found = CHECKSUM_MASK[idx + 4] ^ ((checksum & 0xFF00) >> 8) as u8;

            self.validate_checksum(kind, found as u16, expected as u16)?;
        }

        Ok(None)
    }

    fn validate_checksum(&mut self, kind: String, found: u16, expected: u16) -> Result<Option<()>> {
        let result = if found == expected {
            Ok(())
        } else {
            Err(Error::InvalidChecksum {
                kind,
                found,
                expected,
            })
        };

        self.ok_or_warn(result)
    }
}

fn find_partial_board_checksum<'a>(start: u16, strings: &Strings<'a>) -> u16 {
    // Compute the overall file checksum
    let mut file_checksum = start;

    file_checksum = find_region_checksum(&strings.title_region, file_checksum);
    file_checksum = find_region_checksum(&strings.author_region, file_checksum);
    file_checksum = find_region_checksum(&strings.copyright_region, file_checksum);

    for clue_region in &strings.clue_regions {
        file_checksum = find_region_checksum(clue_region, file_checksum);
    }

    file_checksum = find_region_checksum(&strings.notes_region, file_checksum);
    file_checksum
}

fn find_region_checksum<'a>(region: &Region<'a>, start: u16) -> u16 {
    let mut checksum = start;

    for &byte in region.bytes {
        if checksum & 1 != 0 {
            checksum = (checksum >> 1) + 0x8000;
        } else {
            checksum >>= 1;
        }
        checksum = checksum.wrapping_add(byte as u16);
    }

    checksum
}
