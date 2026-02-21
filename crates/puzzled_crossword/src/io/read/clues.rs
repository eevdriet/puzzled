use puzzled_core::Position;
use std::collections::BTreeMap;

use crate::io::{
    PuzReader, SECTION_SEPARATOR, Span, Strings, TxtReader, TxtState, build_string, format, read,
};
use crate::{Clue, ClueSpec, Clues, Direction, Squares, SquaresExtension};

impl PuzReader {
    pub(crate) fn read_clues(&self, squares: &Squares, strings: &Strings) -> read::Result<Clues> {
        let mut entries = BTreeMap::new();

        let mut num: u8 = 1;
        let mut clues_iter = strings.clues.iter().enumerate();

        let mut start_at_pos = |num: u8, start: Position, direction: Direction| -> bool {
            // Cannot start clue at current position
            if !squares.starts_in_dir(start, direction) {
                return false;
            }

            // No more clues to parse
            let text = match clues_iter.next() {
                None => return false,
                Some((_, clue)) => build_string(clue),
            };
            let len = squares.find_playable_len(start, direction);

            let entry = Clue::new(num, direction, text, start, len);
            entries.insert((num, direction), entry);

            true
        };

        for start in squares.positions() {
            let starts_across = start_at_pos(num, start, Direction::Across);
            let starts_down = start_at_pos(num, start, Direction::Down);

            if starts_across || starts_down {
                num += 1;
            }
        }

        if let Some((idx, clue)) = clues_iter.next() {
            let id = idx as u16 + 1;
            return Err(read::Error {
                span: Span::default(),
                kind: read::ErrorKind::MissingClue {
                    id,
                    clue: build_string(clue),
                },
                context: "Clues".to_string(),
            });
        }

        Ok(Clues::new(entries))
    }
}

impl<'a> TxtReader {
    pub(crate) fn parse_clues(&self, state: &mut TxtState<'a>) -> read::Result<Vec<ClueSpec>> {
        let mut clues = Vec::new();
        let context = "Clues";

        let err = |reason: &str| read::Error {
            span: 0..0,
            kind: format::Error::InvalidClueSpec {
                reason: reason.to_string(),
            }
            .into(),
            context: "Clues".to_string(),
        };

        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }
            if line == SECTION_SEPARATOR {
                break;
            }

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
            let text = state.parse_string(text, context)?;

            // Add the clue
            let clue = ClueSpec::new(direction, text);
            clues.push(clue);
        }

        Ok(clues)
    }
}
