use crate::Puzzle;
use crate::io::{Error, ErrorKind, PuzParser, PuzState, Result, Span, TxtParser, TxtState};

#[derive(Debug)]
pub(crate) struct Strings<'a> {
    pub title: &'a [u8],
    pub author: &'a [u8],
    pub copyright: &'a [u8],
    pub notes: &'a [u8],
    pub clues: Vec<&'a [u8]>,

    pub clues_span: Span,
}

impl<'a> PuzParser {
    pub(crate) fn parse_strings(
        &self,
        clue_count: usize,
        state: &mut PuzState<'a>,
    ) -> Result<Strings<'a>> {
        let title = state.read_str("Title")?;
        let author = state.read_str("Author")?;
        let copyright = state.read_str("copyright")?;

        // Sequentially parse the clues
        let (clues, clues_span) = state.read_span(|s| {
            let mut clues = Vec::with_capacity(clue_count);

            for num in 1..=clue_count {
                let context = format!("Clue #{num}");
                let clue = s.read_str(context)?;

                clues.push(clue);
            }

            Ok(clues)
        })?;

        let notes = state.read_str("Notes")?;

        Ok(Strings {
            title,
            author,
            copyright,
            notes,
            clues,
            clues_span,
        })
    }
}

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
