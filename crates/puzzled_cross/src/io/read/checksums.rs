use crate::io::{
    Grids, Header, PuzReader, Span, Strings, find_cib_checksum, find_file_checksum,
    find_mask_checksums, find_region_checksum, find_strings_checksum,
    read::{self, PuzState},
};

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
