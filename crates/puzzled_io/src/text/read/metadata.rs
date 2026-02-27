use std::str::FromStr;

use puzzled_core::{Metadata, Timer, Version};

use crate::format;
use crate::text::read::{self, TxtState};

impl<'a> TxtState<'a> {
    pub fn read_metadata(
        &mut self,
        separator: Option<&str>,
    ) -> read::Result<(Metadata, Option<Timer>)> {
        let mut metadata = Metadata::default();
        let mut timer = None;

        while let Some(line) = self.next_line() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }

            if let Some(sep) = separator
                && line == sep
            {
                break;
            }

            // Read each property as `<prop>: "<text>"`
            let (prop, text) = line
                .split_once(':')
                .ok_or(read::Error::InvalidMetaProperty {
                    found: line.to_string(),
                    reason: "Property should be formatted as <key>: \"<value>\"".to_string(),
                })?;

            let text = self.read_string(text)?;

            // Try to set a property in the metadata
            match prop.trim().to_ascii_lowercase().as_str() {
                "author" => {
                    metadata = metadata.with_author(text);
                }
                "copyright" => {
                    metadata = metadata.with_copyright(text);
                }
                "notes" => {
                    metadata = metadata.with_notes(text);
                }
                "title" => {
                    metadata = metadata.with_title(text);
                }
                "version" => match Version::new(text.as_bytes()) {
                    Ok(version) => {
                        metadata = metadata.with_version(version);
                    }
                    Err(reason) => {
                        return Err(format::Error::Version(reason).into());
                    }
                },
                "timer" => match Timer::from_str(&text) {
                    Ok(t) => {
                        timer = Some(t);
                    }
                    Err(err) => return Err(format::Error::Timer(err).into()),
                },
                _ => {
                    return Err(read::Error::InvalidMetaProperty {
                        found: prop.to_string(),
                        reason: "Type is unknown".to_string(),
                    });
                }
            }
        }

        Ok((metadata, timer))
    }
}
