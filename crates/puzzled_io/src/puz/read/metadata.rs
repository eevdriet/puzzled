use puzzled_core::{Metadata, Version};

use crate::puz::{ByteStr, Header, Strings};

pub fn read_metadata(header: &Header, strings: &Strings) -> Metadata {
    let mut metadata = Metadata::default();

    let str_or = |str: &ByteStr| (!str.is_empty()).then_some(str.to_string());

    if let Some(author) = str_or(&strings.author) {
        metadata = metadata.with_author(author);
    }
    if let Some(copyright) = str_or(&strings.copyright) {
        metadata = metadata.with_copyright(copyright);
    }
    if let Some(notes) = str_or(&strings.notes) {
        metadata = metadata.with_notes(notes);
    }
    if let Some(title) = str_or(&strings.title) {
        metadata = metadata.with_title(title);
    }

    if let Ok(version) = Version::new(&header.version) {
        metadata = metadata.with_version(version)
    }

    metadata
}
