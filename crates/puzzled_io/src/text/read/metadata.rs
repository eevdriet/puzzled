use std::str::FromStr;

use puzzled_core::{Metadata, Timer, Version};

use crate::format;
use crate::text::read::{self, TxtState};

impl<'a> TxtState<'a> {
    pub fn read_metadata(&mut self, separator: Option<&str>) -> read::Result<Metadata> {
        let mut metadata = Metadata::default();

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
                    metadata.author = Some(text);
                }
                "copyright" => {
                    metadata.author = Some(text);
                }
                "notes" => {
                    metadata.notes = Some(text);
                }
                "title" => {
                    metadata.title = Some(text);
                }
                "version" => match Version::new(text.as_bytes()) {
                    Ok(version) => {
                        metadata.version = Some(version);
                    }
                    Err(reason) => {
                        return Err(format::Error::Version(reason).into());
                    }
                },
                "timer" => match Timer::from_str(&text) {
                    Ok(timer) => metadata.timer = timer,
                    Err(reason) => return Err(format::Error::Timer(reason).into()),
                },
                _ => {
                    return Err(read::Error::InvalidMetaProperty {
                        found: prop.to_string(),
                        reason: "Type is unknown".to_string(),
                    });
                }
            }
        }

        Ok(metadata)
    }
}
