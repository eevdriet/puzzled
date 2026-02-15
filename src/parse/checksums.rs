use crate::parse::{
    Error, ErrorKind, Header, PuzParser, PuzState, PuzzleGrid, Result, Span, Strings,
};

const CHECKSUM_MASK: &[u8; 9] = b"ICHEATED\0";

impl<'a> PuzParser {
    pub(crate) fn validate_file_checksum(
        &self,
        file_span: &Span,
        header: &Header<'a>,
        grid: &PuzzleGrid,
        strings: &Strings<'a>,
        state: &mut PuzState,
    ) -> Result<Option<()>> {
        // Compute the overall file checksum
        let mut file_checksum = header.cib_checksum;

        file_checksum = state.find_span_checksum(&grid.solution_span, file_checksum);
        file_checksum = state.find_span_checksum(&grid.state_span, file_checksum);
        file_checksum = self.find_strings_checksum(strings, file_checksum);

        // Validate the file checksum
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
        strings: &Strings<'a>,
        state: &mut PuzState,
    ) -> Result<Option<()>> {
        // Collect all checksums to mask
        let checksums = [
            state.find_span_checksum(&header.cib_span, 0),    // CIB
            state.find_span_checksum(&grid.solution_span, 0), // Solution
            state.find_span_checksum(&grid.state_span, 0),    // Grid
            self.find_strings_checksum(strings, 0),           // Partial board
        ];

        let span = header.masks_span.clone();

        // Check whether the right amount of masks are present
        if header.low_checksums.len() != checksums.len() {
            return Err(Error {
                span,
                kind: ErrorKind::MissingChecksum {
                    found: header.low_checksums.len(),
                    expected: checksums.len(),
                },
                context: "Masked low checksums".to_string(),
            });
        }

        if header.high_checksums.len() != checksums.len() {
            return Err(Error {
                span,
                kind: ErrorKind::MissingChecksum {
                    found: header.high_checksums.len(),
                    expected: checksums.len(),
                },
                context: "Masked high checksums".to_string(),
            });
        }

        // Validate masked low checksums
        for (idx, (&checksum, &expected)) in checksums
            .iter()
            .zip(header.low_checksums.iter())
            .enumerate()
        {
            let start = span.start + idx;
            let span = start..start + 1;
            let kind = format!("Low mask #{}", idx + 1);
            let found = CHECKSUM_MASK[idx] ^ (checksum & 0xFF) as u8;

            self.validate_checksum(span, kind, found as u16, expected as u16, state)?;
        }

        // Validate against the masked high checksums
        for (idx, (&checksum, &expected)) in checksums
            .iter()
            .zip(header.high_checksums.iter())
            .enumerate()
        {
            let start = span.start + idx + 4;
            let span = start..start + 1;
            let kind = format!("High mask #{}", idx + 1);
            let found = CHECKSUM_MASK[idx + 4] ^ ((checksum & 0xFF00) >> 8) as u8;

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

    fn find_strings_checksum(&self, strings: &Strings<'a>, start: u16) -> u16 {
        // Compute the overall file checksum
        let mut file_checksum = start;

        file_checksum = find_str_checksum(strings.title, file_checksum, true);
        file_checksum = find_str_checksum(strings.author, file_checksum, true);
        file_checksum = find_str_checksum(strings.copyright, file_checksum, true);

        for &clue in &strings.clues {
            let clue_without_end = match clue.strip_suffix(&[0]) {
                None => clue,
                Some(prefix) => prefix,
            };

            file_checksum = find_str_checksum(clue_without_end, file_checksum, false);
        }

        file_checksum = find_str_checksum(strings.notes, file_checksum, true);
        file_checksum
    }
}

impl<'a> PuzState<'a> {
    fn find_span_checksum(&self, span: &Span, start: u16) -> u16 {
        let region = &self.input[span.start..span.end];

        find_region_checksum(region, start)
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
