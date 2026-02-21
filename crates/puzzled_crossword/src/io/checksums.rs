use crate::io::Strings;

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
