use std::collections::BTreeMap;

use crate::{Entry, EntryId, Error, Parser, PuzzleGrid, Result, parse_string};

impl<'a> Parser<'a> {
    pub(crate) fn parse_entries(
        &mut self,
        grid: &'a PuzzleGrid,
        flat_clues: &'a [&'a [u8]],
    ) -> Result<BTreeMap<EntryId, Entry>> {
        let mut entries = BTreeMap::new();

        let mut num: u8 = 1;
        let mut clues_iter = flat_clues.iter();

        for start in grid.solution.positions() {
            let starts_across = grid.starts_across(start);
            let starts_down = grid.starts_down(start);

            if starts_across {
                let id = EntryId::Across(num);
                let clue = clues_iter.next().ok_or(Error::MissingClue { id })?;

                let entry = Entry {
                    id,
                    start,
                    len: grid.find_playable_len(start, id.direction()),
                    clue: parse_string(clue),
                };
                entries.insert(id, entry);
            }

            if starts_down {
                let id = EntryId::Down(num);
                let clue = clues_iter.next().ok_or(Error::MissingClue { id })?;

                let entry = Entry {
                    id,
                    start,
                    len: grid.find_playable_len(start, id.direction()),
                    clue: parse_string(clue),
                };
                entries.insert(id, entry);
            }

            if starts_across || starts_down {
                num += 1;
            }
        }

        Ok(entries)
    }
}
