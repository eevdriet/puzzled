use crate::Puzzle;
use crate::io::{Context, PuzRead, Strings, TxtReader, TxtState, read};

impl Strings {
    pub(crate) fn read_from<R: PuzRead>(reader: &mut R, clue_count: u16) -> read::Result<Self> {
        let title = reader.read_str0().context("Title")?;
        let author = reader.read_str0().context("Author")?;
        let copyright = reader.read_str0().context("Copyright")?;

        // Sequentially parse the clues
        let mut clues = Vec::with_capacity(clue_count as usize);

        for num in 1..=clue_count {
            let context = format!("Clue #{num}");
            let clue = reader.read_str0().context(context)?;

            clues.push(clue);
        }

        let notes = reader.read_str0().context("Notes")?;

        Ok(Strings {
            title,
            author,
            copyright,
            notes,
            clues,
        })
    }
}

impl<'a> TxtReader {
    pub(crate) fn parse_strings(
        &self,
        mut puzzle: Puzzle,
        state: &mut TxtState<'a>,
    ) -> read::Result<Puzzle> {
        let context = "Metadata";

        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }

            let (prop, text) = line.split_once(':').ok_or(read::Error {
                span: 0..0,
                kind: read::ErrorKind::Custom(format!("Invalid metadata: {line}")),
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
                        return Err(read::Error {
                            span: 0..0,
                            kind: read::ErrorKind::Custom(format!("Invalid metadata: {line}")),
                            context: context.to_string(),
                        });
                    }
                },
                _ => {
                    return Err(read::Error {
                        span: 0..0,
                        kind: read::ErrorKind::Custom(format!("Invalid metadata property: {prop}")),
                        context: context.to_string(),
                    });
                }
            }
        }

        Ok(puzzle)
    }
}
