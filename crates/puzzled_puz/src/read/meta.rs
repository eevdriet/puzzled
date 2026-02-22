use puzzled_core::{Metadata, Version};

use crate::{ByteStr, Extras, Header, Strings, build_string};

pub fn read_metadata(header: &Header, strings: &Strings, extras: &Extras) -> Metadata {
    let mut metadata = Metadata::default();

    let str_or = |str: &ByteStr| {
        let string = build_string(str);
        (!string.is_empty()).then_some(string)
    };

    metadata.author = str_or(&strings.author);
    metadata.copyright = str_or(&strings.copyright);
    metadata.notes = str_or(&strings.notes);
    metadata.title = str_or(&strings.title);

    if let Ok(version) = Version::new(&header.version) {
        metadata.version = Some(version);
    }

    if let Some(timer) = &extras.ltim {
        metadata.timer = *timer;
    }

    metadata
}
