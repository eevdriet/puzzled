use std::collections::BTreeMap;

use crate::io::{
    Error, ErrorKind, PuzParser, PuzState, Result, SECTION_SEPARATOR, Strings, TxtParser, TxtState,
};
use crate::{Clue, ClueId, ClueSpec, Direction, Grid, Position, Square};

impl<'a> PuzParser {
    pub(crate) fn read_clues(
        &self,
        grid: &Grid<Square>,
        strings: &Strings<'a>,
    ) -> Result<BTreeMap<ClueId, Clue>> {
        let mut entries = BTreeMap::new();

        let mut num: u8 = 1;
        let mut clues_iter = strings.clues.iter().enumerate();

        let mut start_at_pos = |num: u8, start: Position, direction: Direction| -> bool {
            // Cannot start clue at current position
            if !grid.starts_in_dir(start, direction) {
                return false;
            }

            // No more clues to parse
            let text = match clues_iter.next() {
                None => return false,
                Some((_, clue)) => PuzState::build_string(clue),
            };
            let len = grid.find_playable_len(start, direction);

            let entry = Clue::new(num, direction, text, start, len);
            entries.insert((num, direction), entry);

            true
        };

        for start in grid.positions() {
            let starts_across = start_at_pos(num, start, Direction::Across);
            let starts_down = start_at_pos(num, start, Direction::Down);

            if starts_across || starts_down {
                num += 1;
            }
        }

        if let Some((idx, clue)) = clues_iter.next() {
            let id = idx as u16 + 1;
            return Err(Error {
                span: strings.clues_span.clone(),
                kind: ErrorKind::MissingClue {
                    id,
                    clue: PuzState::build_string(clue),
                },
                context: "Clues".to_string(),
            });
        }

        Ok(entries)
    }
}

impl<'a> TxtParser {
    pub(crate) fn parse_clues(&self, state: &mut TxtState<'a>) -> Result<Vec<ClueSpec>> {
        let mut clues = Vec::new();
        let context = "Clues";

        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }
            if line == SECTION_SEPARATOR {
                break;
            }

            let (dir_str, text) = line.split_once(':').ok_or(Error {
                span: 0..0,
                kind: ErrorKind::Custom(format!("Invalid clue spec: {line}")),
                context: context.to_string(),
            })?;

            // Validate the direction of the clue
            let direction = match dir_str.trim() {
                "A" => Direction::Across,
                "D" => Direction::Down,
                _ => {
                    return Err(Error {
                        span: 0..0,
                        kind: ErrorKind::Custom(format!("Invalid clue spec: {line}")),
                        context: context.to_string(),
                    });
                }
            };
            eprintln!("Text to parse to string: '{text}'");

            // Validate the clue text
            let text = state.parse_string(text, context)?;

            // Add the clue
            let clue = ClueSpec::new(direction, text);
            clues.push(clue);
        }

        Ok(clues)
    }
}
