use crate::io::{
    Grids, Header, PuzWriter, Strings, find_cib_checksum, find_file_checksum, find_mask_checksums,
    find_region_checksum, find_strings_checksum,
};

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
