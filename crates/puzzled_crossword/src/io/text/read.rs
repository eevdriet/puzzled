use std::str::FromStr;

use puzzled_core::{Grid, GridError, Metadata, Timer, Version, format};

use crate::{ClueSpec, Crossword, CrosswordCell, Direction, Square, Squares, io::TxtState};

#[derive(Debug, Default)]
pub struct TxtReader;

impl<'a> TxtReader {
    pub fn read(&self, input: &'a str) -> format::Result<Crossword> {
        let mut state = TxtState::new(input);

        let squares = self.parse_grid(&mut state)?;
        let clues = self.parse_clues(&mut state)?;
        let meta = self.parse_metadata(&mut state)?;

        let mut puzzle = Crossword::from_squares(squares, meta);

        eprintln!("Clues: {clues:?}");
        puzzle.insert_clues(clues);

        Ok(puzzle)
    }

    pub(crate) fn parse_metadata(&self, state: &mut TxtState<'a>) -> format::Result<Metadata> {
        let mut meta = Metadata::default();

        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }

            let (prop, text) = line.split_once(':').ok_or(format::Error::InvalidProperty {
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
                    meta.author = Some(text);
                }
                "copyright" => {
                    meta.author = Some(text);
                }
                "notes" => {
                    meta.notes = Some(text);
                }
                "title" => {
                    meta.title = Some(text);
                }
                "version" => match Version::new(text.as_bytes()) {
                    Ok(version) => {
                        meta.version = Some(version);
                    }
                    Err(reason) => {
                        return Err(format::Error::Version(reason));
                    }
                },
                "timer" => match Timer::from_str(&text) {
                    Ok(timer) => meta.timer = timer,
                    Err(reason) => return Err(format::Error::Timer(reason)),
                },
                _ => {
                    return Err(format::Error::InvalidProperty {
                        found: prop.to_string(),
                        reason: "Type is unknown".to_string(),
                    });
                }
            }
        }

        Ok(meta)
    }

    pub(crate) fn parse_grid(&self, state: &mut TxtState<'a>) -> format::Result<Squares> {
        let mut squares = Vec::new();

        let err = |err: GridError| format::Error::Grid(err);

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

    fn parse_row(row: u8, line: &str) -> format::Result<Vec<Square>> {
        if !line.starts_with('[') || !line.ends_with(']') {
            let err = GridError::InvalidRow {
                row,
                reason: "Should be delimited by [...]".to_string(),
            };

            return Err(format::Error::Grid(err));
        }

        let line = &line[1..line.len() - 1];
        let mut squares = Vec::new();

        for token in line.split_whitespace() {
            let square = match token {
                "." => None,
                word if word.len() == 1 => Some(CrosswordCell::letter(
                    word.chars().next().expect("Word is not empty"),
                )),
                rebus => Some(CrosswordCell::rebus(rebus.to_string())),
            };

            squares.push(square);
        }

        Ok(squares)
    }

    pub(crate) fn parse_clues(&self, state: &mut TxtState<'a>) -> format::Result<Vec<ClueSpec>> {
        let mut clues = Vec::new();

        let err = |reason: &str| format::Error::ClueSpec {
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
