use crate::Puzzle;
use crate::io::{Error, ErrorKind, PuzParser, PuzState, Result, Strings, TxtParser, TxtState};

impl<'a> TxtParser {
    pub(crate) fn parse_strings(
        &self,
        mut puzzle: Puzzle,
        state: &mut TxtState<'a>,
    ) -> Result<Puzzle> {
        let context = "Metadata";

        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }

            let (prop, text) = line.split_once(':').ok_or(Error {
                span: 0..0,
                kind: ErrorKind::Custom(format!("Invalid metadata: {line}")),
                context: context.to_string(),
            })?;

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
                    [x, b'.', y] if x.is_ascii_digit() && y.is_ascii_digit() => {
                        puzzle = puzzle.with_version(text);
                    }
                    _ => {
                        return Err(Error {
                            span: 0..0,
                            kind: ErrorKind::Custom(format!("Invalid metadata: {line}")),
                            context: context.to_string(),
                        });
                    }
                },
                _ => {
                    return Err(Error {
                        span: 0..0,
                        kind: ErrorKind::Custom(format!("Invalid metadata property: {prop}")),
                        context: context.to_string(),
                    });
                }
            }
        }

        Ok(puzzle)
    }
}
