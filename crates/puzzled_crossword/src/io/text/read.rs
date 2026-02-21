use puzzled_core::{Grid, GridError, Version};

use crate::{
    Cell, ClueSpec, Crossword, Direction, Square, Squares,
    io::{TxtState, text},
};

#[derive(Debug, Default)]
pub struct TxtReader;

impl<'a> TxtReader {
    pub fn read(&self, input: &'a str) -> text::Result<Crossword> {
        let mut state = TxtState::new(input);

        let squares = self.parse_grid(&mut state)?;
        let mut puzzle = Crossword::from_squares(squares);

        let clues = self.parse_clues(&mut state)?;
        eprintln!("Clues: {clues:?}");
        puzzle.insert_clues(clues);

        puzzle = self.parse_strings(puzzle, &mut state)?;

        Ok(puzzle)
    }

    pub(crate) fn parse_strings(
        &self,
        mut puzzle: Crossword,
        state: &mut TxtState<'a>,
    ) -> text::Result<Crossword> {
        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }

            let (prop, text) = line.split_once(':').ok_or(text::Error::InvalidProperty {
                found: line.to_string(),
                reason: "Property should be formatted as <key>: \"<value>\"".to_string(),
            })?;

            // Parse timer separately
            if prop == "timer" {
                continue;
            }

            // Validate the clue text
            let text = state.parse_string(text)?;

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
                "version" => match Version::new(text.as_bytes()) {
                    Ok(version) => {
                        puzzle = puzzle.with_version(version);
                    }
                    Err(reason) => {
                        return Err(text::Error::InvalidVersion { reason });
                    }
                },
                _ => {
                    return Err(text::Error::InvalidProperty {
                        found: prop.to_string(),
                        reason: "Type is unknown".to_string(),
                    });
                }
            }
        }

        Ok(puzzle)
    }

    pub(crate) fn parse_grid(&self, state: &mut TxtState<'a>) -> text::Result<Squares> {
        let mut squares = Vec::new();

        let err = |err: GridError| text::Error::Grids(err);

        let mut cols = None;
        let mut rows = 0;

        while let Some(line) = state.peek() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                state.next();
                continue;
            }

            if !line.starts_with("[") {
                break;
            }

            let line = state.next().expect("Already peeked").trim();
            rows += 1;

            // Parse the next row and verify its width
            let row = Self::parse_row(rows, line)?;
            let row_width = row.len() as u8;
            squares.extend(row);

            if let Some(width) = cols {
                if width != row_width {
                    return Err(err(GridError::InvalidWidth {
                        row: rows,
                        found: row_width,
                        expected: width,
                    }));
                }
            } else {
                cols = Some(row_width);
            }
        }

        let cols = cols.ok_or(err(GridError::InvalidDimensions { rows, cols: 0 }))?;
        let squares = Grid::from_vec(squares, cols as usize)?;

        Ok(Squares::new(squares))
    }

    fn parse_row(row: u8, line: &str) -> text::Result<Vec<Square>> {
        if !line.starts_with('[') || !line.ends_with(']') {
            return Err(text::Error::Grids(GridError::InvalidRow {
                row,
                reason: "Should be delimited by [...]".to_string(),
            }));
        }

        let line = &line[1..line.len() - 1];
        let mut squares = Vec::new();

        for token in line.split_whitespace() {
            let square = match token {
                "." => None,
                word if word.len() == 1 => Some(Cell::letter(
                    word.chars().next().expect("Word is not empty"),
                )),
                rebus => Some(Cell::rebus(rebus.to_string())),
            };

            squares.push(square);
        }

        Ok(squares)
    }

    pub(crate) fn parse_clues(&self, state: &mut TxtState<'a>) -> text::Result<Vec<ClueSpec>> {
        let mut clues = Vec::new();

        let err = |reason: &str| text::Error::InvalidClueSpec {
            reason: reason.to_string(),
        };

        while let Some(line) = state.peek() {
            let line = line.trim();
            eprintln!("Clue line: {line}");

            // Skip empty lines and stop parsing when no more list items `-` found
            if line.is_empty() {
                state.next();
                continue;
            }

            if !line.starts_with("-") {
                break;
            }

            let line = state.next().expect("Already peeked").trim();
            let line = match line.strip_prefix("-") {
                Some(line) => line,
                _ => break,
            };

            // Validate direction/text separation
            let (dir_str, text) = line
                .split_once(':')
                .ok_or(err("Clues should be specified as <dir> : <text>"))?;

            // Validate the direction of the clue
            let direction = match dir_str.trim() {
                "A" => Direction::Across,
                "D" => Direction::Down,
                _ => {
                    return Err(err(
                        "Clue direction should be either A (across) or D (down)",
                    ));
                }
            };

            // Validate the clue text
            let text = state.parse_string(text)?;

            // Add the clue
            let clue = ClueSpec::new(direction, text);
            clues.push(clue);
        }

        Ok(clues)
    }
}
