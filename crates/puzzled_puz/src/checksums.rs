use crate::{Grids, Header, PuzReader, PuzState, PuzWriter, Span, Strings, read};

#[doc(hidden)]
pub fn find_region_checksum(region: &[u8], start: u16) -> u16 {
    let mut checksum = start;

    for &byte in region {
        if checksum & 1 != 0 {
            checksum = (checksum >> 1) + 0x8000;
        } else {
            checksum >>= 1;
        }
        checksum = checksum.wrapping_add(byte as u16);
    }

    checksum
}

#[doc(hidden)]
pub(crate) fn find_str_checksum(str: &[u8], start: u16, ignore_empty: bool) -> u16 {
    if ignore_empty && matches!(str, [] | [b'\0']) {
        return start;
    }

    find_region_checksum(str, start)
}

pub(crate) fn find_cib_checksum(cib_region: &[u8]) -> u16 {
    find_region_checksum(cib_region, 0)
}

pub(crate) fn find_file_checksum<'a>(
    cib_checksum: u16,
    solution_region: &'a [u8],
    state_region: &'a [u8],
    strings: &Strings,
) -> u16 {
    // Compute the overall file checksum
    let mut file_checksum = cib_checksum;

    file_checksum = find_region_checksum(solution_region, file_checksum);
    file_checksum = find_region_checksum(state_region, file_checksum);
    file_checksum = find_strings_checksum(strings, file_checksum);

    file_checksum
}

#[doc(hidden)]
pub fn find_strings_checksum(strings: &Strings, start: u16) -> u16 {
    // Compute the overall file checksum
    let mut file_checksum = start;

    file_checksum = find_str_checksum(&strings.title, file_checksum, true);
    file_checksum = find_str_checksum(&strings.author, file_checksum, true);
    file_checksum = find_str_checksum(&strings.copyright, file_checksum, true);

    for clue in &strings.clues {
        let clue_without_end = match clue.strip_suffix(&[0]) {
            None => clue,
            Some(prefix) => &prefix.to_vec(),
        };

        file_checksum = find_str_checksum(clue_without_end, file_checksum, false);
    }

    file_checksum = find_str_checksum(&strings.notes, file_checksum, true);
    file_checksum
}

pub(crate) fn find_mask_checksums(
    cib_checksum: u16,
    solution_checksum: u16,
    state_checksum: u16,
    strings_checksum: u16,
) -> [u8; 8] {
    [
        b'I' ^ (cib_checksum & 0xFF) as u8,
        b'C' ^ (solution_checksum & 0xFF) as u8,
        b'H' ^ (state_checksum & 0xFF) as u8,
        b'E' ^ (strings_checksum & 0xFF) as u8,
        b'A' ^ ((cib_checksum & 0xFF00) >> 8) as u8,
        b'T' ^ ((solution_checksum & 0xFF00) >> 8) as u8,
        b'E' ^ ((state_checksum & 0xFF00) >> 8) as u8,
        b'D' ^ ((strings_checksum & 0xFF00) >> 8) as u8,
    ]
}

impl PuzReader {
    pub(crate) fn validate_checksums(
        &self,
        header: &Header,
        grids: &Grids,
        strings: &Strings,
        state: &mut PuzState,
    ) -> read::Result<()> {
        // CIB
        let cib_checksum = find_cib_checksum(&header.cib_region);
        self.validate_checksum("CIB".to_string(), cib_checksum, header.cib_checksum, state)?;

        // File
        let solution_region = grids.solution.data();
        let state_region = grids.state.data();

        let file_checksum =
            find_file_checksum(cib_checksum, solution_region, state_region, strings);

        self.validate_checksum(
            "File".to_string(),
            file_checksum,
            header.file_checksum,
            state,
        )?;

        // Masks
        let solution_checksum = find_region_checksum(solution_region, 0);
        let state_checksum = find_region_checksum(state_region, 0);
        let strings_checksum = find_strings_checksum(strings, 0);

        let mask_checksums = find_mask_checksums(
            cib_checksum,
            solution_checksum,
            state_checksum,
            strings_checksum,
        );

        for (idx, (&found, expected)) in
            mask_checksums.iter().zip(header.mask_checksums).enumerate()
        {
            let kind = format!(
                "{} mask #{}",
                if idx < 4 { "Low" } else { "High" },
                (idx % 4) + 1
            );

            self.validate_checksum(kind, found as u16, expected as u16, state)?;
        }

        Ok(())
    }

    fn validate_checksum(
        &self,
        context: String,
        found: u16,
        expected: u16,
        state: &mut PuzState,
    ) -> read::Result<Option<()>> {
        let result = (found == expected).then_some(()).ok_or(read::Error {
            span: Span::default(),
            kind: read::ErrorKind::InvalidChecksum { found, expected },
            context,
        });

        state.ok_or_warn(result)
    }
}

impl PuzWriter {
    pub(crate) fn write_checksums(&self, header: &mut Header, grids: &Grids, strings: &Strings) {
        // CIB
        let cib_checksum = find_cib_checksum(&header.cib_region);
        header.cib_checksum = cib_checksum;

        // File
        let solution_region = grids.solution.data();
        let state_region = grids.state.data();

        header.file_checksum =
            find_file_checksum(cib_checksum, solution_region, state_region, strings);

        // Masks
        let solution_checksum = find_region_checksum(solution_region, 0);
        let state_checksum = find_region_checksum(state_region, 0);
        let strings_checksum = find_strings_checksum(strings, 0);

        header.mask_checksums = find_mask_checksums(
            cib_checksum,
            solution_checksum,
            state_checksum,
            strings_checksum,
        );
    }
}
