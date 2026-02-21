use crate::Crossword;
use crate::io::{Context, PuzRead, Strings, TxtReader, TxtState, format, is_valid_version, read};

impl<'a> TxtReader {
    pub(crate) fn parse_strings(
        &self,
        mut puzzle: Crossword,
        state: &mut TxtState<'a>,
    ) -> read::Result<Crossword> {
        let context = "Metadata";

        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }

            let (prop, text) = line
                .split_once(':')
                .ok_or(format::Error::InvalidProperty {
                    found: line.to_string(),
                    reason: "Property should be formatted as <key>: \"<value>\"".to_string(),
                })
                .context(context)?;

            // Parse timer separately
            if prop == "timer" {
                continue;
            }

            // Validate the clue text
            let text = state.parse_string(text, context)?;

            match prop.to_ascii_lowercase().as_str() {
                "author" => {
                    puzzle = puzzle.with_author(text);
                }
                "copyright" => {
                    puzzle = puzzle.with_copyright(text);
                }
                "notes" => {
                    puzzle = puzzle.with_notes(text);
                }
                "title" => {
                    puzzle = puzzle.with_title(text);
                }
                "version" => match text.as_bytes() {
                    version if is_valid_version(version) => {
                        puzzle = puzzle.with_version(text);
                    }
                    _ => {
                        return Err(format::Error::InvalidVersion).context(context);
                    }
                },
                _ => {
                    return Err(format::Error::InvalidProperty {
                        found: prop.to_string(),
                        reason: "Type is unknown".to_string(),
                    })
                    .context(context);
                }
            }
        }

        Ok(puzzle)
    }
}
