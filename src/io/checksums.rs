use crate::io::{Error, ErrorKind, Header, PuzParser, PuzState, PuzzleGrid, Result, Span, Strings};

impl<'a> PuzParser {
    pub(crate) fn validate_file_checksum(
        &self,
        header: &Header<'a>,
        grid: &PuzzleGrid,
        strings: &Strings,
        file_span: Span,
        state: &mut PuzState,
    ) -> Result<Option<()>> {
        let solution_region = state.region(grid.solution_span.clone());
        let state_region = state.region(grid.state_span.clone());
        let file_checksum =
            find_file_checksum(header.cib_checksum, solution_region, state_region, strings);

        self.validate_checksum(
            file_span.clone(),
            "file".to_string(),
            file_checksum,
            header.file_checksum,
            state,
        )
    }

    pub(crate) fn validate_masked_checksums(
        &self,
        header: &Header<'a>,
        grid: &PuzzleGrid,
        strings: &Strings,
        state: &mut PuzState,
    ) -> Result<Option<()>> {
        let cib_checksum = find_region_checksum(state.region(header.cib_span.clone()), 0);
        let solution_checksum = find_region_checksum(state.region(grid.solution_span.clone()), 0);
        let state_checksum = find_region_checksum(state.region(grid.state_span.clone()), 0);
        let strings_checksum = find_strings_checksum(strings, 0);

        let span = header.masks_span.clone();

        let mask_checksums = find_mask_checksums(
            cib_checksum,
            solution_checksum,
            state_checksum,
            strings_checksum,
        );

        for (idx, (&found, &expected)) in
            mask_checksums.iter().zip(header.mask_checksums).enumerate()
        {
            let start = span.start + idx;
            let span = start..start + 1;
            let kind = format!(
                "{} mask #{}",
                if idx < 4 { "Low" } else { "High" },
                (idx % 4) + 1
            );

            self.validate_checksum(span, kind, found as u16, expected as u16, state)?;
        }

        Ok(None)
    }

    pub(crate) fn validate_checksum(
        &self,
        span: Span,
        kind: String,
        found: u16,
        expected: u16,
        state: &mut PuzState,
    ) -> Result<Option<()>> {
        let result = (found == expected).then_some(()).ok_or(Error {
            span,
            kind: ErrorKind::InvalidChecksum { found, expected },
            context: kind,
        });

        self.ok_or_warn(result, state)
    }
}

pub(crate) fn find_region_checksum(region: &[u8], start: u16) -> u16 {
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

pub(crate) fn find_strings_checksum<'a>(strings: &Strings, start: u16) -> u16 {
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

        file_checksum = find_str_checksum(&clue_without_end, file_checksum, false);
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
