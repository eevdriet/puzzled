use std::collections::BTreeMap;

use puzzled_core::{Cell, Grid, MISSING_ENTRY_CHAR, Metadata, NON_PLAYABLE_CHAR, Position, Square};
use puzzled_io::{
    Context,
    puz::{
        BinaryPuzzle, ByteStr, Extras, Grids, Header, PuzSizeCheck, Span, Strings, WriteStateGrid,
        check_puz_size,
        read::{self, read_metadata},
        windows_1252_to_char,
        write::{self, WriteStyleGrid},
    },
};

use crate::{
    Clue, ClueDirection, Clues, Crossword, CrosswordSquares, CrosswordState, Entry, Solution,
    Squares,
};

impl PuzSizeCheck for Crossword {
    fn check_puz_size(&self) -> write::Result<()> {
        let squares = self.squares();
        let clues = self.clues();

        // Squares grid is of valid size
        squares.check_puz_size()?;

        // Clue count fits into a u16
        check_puz_size("Clues", clues.len(), u16::MAX as usize)?;

        Ok(())
    }
}

impl BinaryPuzzle<CrosswordState> for Crossword {
    fn width(&self) -> usize {
        self.squares().cols()
    }

    fn height(&self) -> usize {
        self.squares().rows()
    }

    fn clues(&self) -> Vec<ByteStr> {
        self.clues()
            .values()
            .map(|clue| {
                let bytes = clue.text().as_bytes();
                ByteStr::new(bytes)
            })
            .collect()
    }

    fn grids(&self, state: &CrosswordState) -> write::Result<(Grid<u8>, Grid<u8>)> {
        let solution = state
            .solutions
            .write_state_grid(|sol| sol.first_letter() as u8);
        let state = state
            .entries
            .write_state_grid(|sol| sol.first_letter() as u8);

        Ok((solution, state))
    }

    fn metadata(&self) -> Option<&Metadata> {
        Some(self.meta())
    }

    fn extras(&self, state: &CrosswordState) -> write::Result<Extras> {
        let squares = self.squares();
        squares.check_puz_size()?;

        let entries = &state.entries;

        let mut extras = Extras::default();

        // GRBS / RTBL
        if squares
            .iter_fills()
            .any(|cell| cell.solution.as_ref().is_some_and(|sol| sol.is_rebus()))
        {
            let mut rebuses: BTreeMap<u8, String> = BTreeMap::new();
            let mut num = 0;

            let grbs = squares.map_ref(|square| {
                match square.as_ref().and_then(|cell| cell.solution.as_ref()) {
                    Some(solution) if solution.is_rebus() => {
                        num += 1;
                        rebuses.insert(num, solution.to_string());

                        num
                    }
                    _ => 0,
                }
            });

            extras.grbs = Some(grbs);
            extras.rtbl = Some(rebuses);
        }

        // LTIM
        // TODO: add back timer extras.ltim = Some(state.timer());

        // GEXT
        let gext = squares.write_combined_style(entries);
        extras.gext = Some(gext);

        Ok(extras)
    }

    fn read_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<(Self, CrosswordState)> {
        // Build the puzzle with owned data
        let (squares, state) = read_state(&grids, &extras)?;

        let clues = read_clues(&squares, &strings)?;
        let meta = read_metadata(&header, &strings);

        let crossword = Crossword::new(squares, clues, meta);
        Ok((crossword, state))
    }
}

fn read_state(grids: &Grids, extras: &Extras) -> read::Result<(Squares, CrosswordState)> {
    grids.validate().context("Squares grids")?;
    let cols = grids.width as usize;

    let (squares, entries) = grids
        .solution
        .iter_indexed()
        .zip(grids.state.iter())
        .map(|((pos, &solution), &state)| {
            let style = extras.get_style(pos);

            let square = match windows_1252_to_char(solution) {
                NON_PLAYABLE_CHAR => Square::new_empty(),
                letter => {
                    let cell = match letter {
                        MISSING_ENTRY_CHAR => Cell::default_with_style(style),
                        letter => {
                            let solution = match extras.get_rebus(pos) {
                                Some(rebus) => Solution::Rebus(rebus.clone()),
                                None => Solution::Letter(letter),
                            };

                            Cell::new_with_style(Some(solution), style)
                        }
                    };

                    Square::new(cell)
                }
            };

            let entry = match windows_1252_to_char(state) {
                NON_PLAYABLE_CHAR => Square::new_empty(),
                letter => {
                    let mut entry = Entry::default_with_style(style);

                    if letter != MISSING_ENTRY_CHAR {
                        let solution = Solution::Letter(letter);
                        entry.enter(solution);
                    }

                    Square::new(entry)
                }
            };

            (square, entry)
        })
        .unzip();

    let squares = Grid::from_vec(squares, cols).expect("Read correct length squares");
    let solutions = squares.map_ref(|square| square.map_ref(|cell| Some(cell.solution.clone())));

    let entries = Grid::from_vec(entries, cols).expect("Read correct lenght entries");

    // TODO: add back timer let timer = extras.ltim.unwrap_or_default();
    let state = CrosswordState::new(solutions, entries);

    Ok((squares, state))
}

fn read_clues(squares: &Squares, strings: &Strings) -> read::Result<Clues> {
    let mut entries = BTreeMap::new();

    let mut num: u8 = 1;
    let mut clues_iter = strings.clues.iter().enumerate();

    let mut start_at_pos = |num: u8, start: Position, direction: ClueDirection| -> bool {
        // Cannot start clue at current position
        if !squares.can_clue_start_in_dir(start, direction) {
            return false;
        }

        // No more clues to parse
        let text = match clues_iter.next() {
            None => return false,
            Some((_, clue)) => clue.to_string(),
        };
        let len = squares.find_clue_len(start, direction);

        let entry = Clue::new(num, direction, text, start, len);
        entries.insert((num, direction).into(), entry);

        true
    };

    for start in squares.positions() {
        let starts_across = start_at_pos(num, start, ClueDirection::Across);
        let starts_down = start_at_pos(num, start, ClueDirection::Down);

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
                clue: clue.to_string(),
            },
            context: "Clues".to_string(),
        });
    }

    Ok(Clues::new(entries))
}

#[cfg(all(test, feature = "puz"))]
mod tests {
    use crate::{Crossword, CrosswordState};
    use puzzled_io::puz::{PuzReader, read};
    use rstest::rstest;
    use std::fs::File;
    use std::path::PathBuf;

    fn parse_puz(
        path: PathBuf,
        strict: bool,
    ) -> read::Result<(Crossword, CrosswordState, Vec<read::Warning>)> {
        let mut file = File::open(path).expect("puzzle file exists");
        let parser = PuzReader::new(strict);

        parser.read_with_warnings(&mut file)
    }

    #[rstest]
    fn parse_ok_puz(#[files("puzzles/ok/*.puz")] path: PathBuf) {
        let result = parse_puz(path, false);
        let (puzzle, _, _) = result.expect("puzzle is parsed correctly");

        assert!(puzzle.rows() > 0);
        assert!(puzzle.cols() > 0);
    }

    #[rstest]
    fn parse_err_puz(#[files("puzzles/err/*.puz")] path: PathBuf) {
        let result = parse_puz(path, true);
        let err = result.expect_err("puzzle is not parsed correctly");

        eprintln!("{err}");
    }

    #[rstest]
    fn parse_warn(#[files("puzzles/warn/*.puz")] path: PathBuf) {
        let result = parse_puz(path, false);
        let (_, _, warnings) = result.expect("puzzle is parsed correctly");

        assert!(!warnings.is_empty());
    }
}
