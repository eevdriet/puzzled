use std::collections::BTreeMap;

use puzzled_core::{Cell, Grid, Position, Square};
use puzzled_io::puz::{
    BinaryPuzzle, ByteStr, Extras, Grids, Header, MISSING_ENTRY_CHAR, NON_PLAYABLE_CHAR,
    PuzSizeCheck, Span, Strings, check_puz_size,
    read::{self, read_metadata},
    windows_1252_to_char, write,
};

use crate::{Clue, Clues, Crossword, CrosswordState, Direction, Entry, Solution, Squares};

impl PuzSizeCheck for Clues {
    fn check_puz_size(&self) -> write::Result<()> {
        let size = self.len();
        let max_size = u16::MAX as usize;

        check_puz_size("Clues", size, max_size)
    }
}

impl BinaryPuzzle for Crossword {
    fn write_header(&self, _state: &CrosswordState) -> write::Result<Header> {
        let mut header = Header::default();

        // Grids
        let squares = self.squares();
        squares.check_puz_size()?;
        header.width = squares.cols() as u8;
        header.height = squares.rows() as u8;

        // Clues
        let clues = self.clues();
        clues.check_puz_size()?;
        header.clue_count = clues.len() as u16;

        // Metadata
        let version = self
            .meta()
            .version()
            .map(|v| v.as_bytes())
            .unwrap_or_default();
        header.version = version;
        header.write_cib();

        Ok(header)
    }

    fn write_grids(&self, state: &CrosswordState) -> write::Result<Grids> {
        // Get the squares and check for overflow of their size
        let squares = self.squares();
        squares.check_puz_size()?;

        let width = squares.rows() as u8;
        let height = squares.cols() as u8;

        // Write the individual grids from the squares
        let solution = state.solution().map_ref(|square| match square.inner() {
            None => NON_PLAYABLE_CHAR,
            Some(solution) => solution
                .as_ref()
                .map(|solution| solution.first_letter())
                .unwrap_or(MISSING_ENTRY_CHAR),
        } as u8);

        let state = state.entries().map_ref(|square| match square.inner() {
            None => NON_PLAYABLE_CHAR,
            Some(entry) => match entry.entry() {
                Some(solution) => solution.first_letter(),
                _ => MISSING_ENTRY_CHAR,
            },
        } as u8);

        // Construct the result and validate
        let grids = Grids {
            solution,
            state,
            width,
            height,
        };

        Ok(grids)
    }

    fn write_strings(&self, _state: &CrosswordState) -> write::Result<Strings> {
        let clues = self.clues();
        clues.check_puz_size()?;

        let mut strings = Strings::from_metadata(self.meta());
        strings.clues = Vec::with_capacity(clues.len());

        for (idx, clue) in clues.values().enumerate() {
            let byte_str = ByteStr::new(clue.text().as_bytes());
            strings.clues[idx] = byte_str;
        }

        Ok(strings)
    }

    fn write_extras(&self, state: &CrosswordState) -> write::Result<Extras> {
        let squares = self.squares();
        squares.check_puz_size()?;

        let mut extras = Extras::default();

        // GRBS / RTBL
        if squares
            .iter_fills()
            .any(|cell| cell.solution.as_ref().is_some_and(|sol| sol.is_rebus()))
        {
            let mut rebuses: BTreeMap<u8, String> = BTreeMap::new();
            let mut num = 0;

            let grbs = squares.map_ref(|square| {
                match square
                    .inner()
                    .as_ref()
                    .and_then(|cell| cell.solution.as_ref())
                {
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
        extras.ltim = Some(state.timer());

        // GEXT
        let entries = state.entries();
        entries.check_puz_size()?;

        let gext: Vec<_> = squares
            .iter()
            .zip(entries.iter())
            .map(|(puzzle_square, entry_square)| {
                let puzzle_style = puzzle_square
                    .inner()
                    .as_ref()
                    .map(|sq| sq.style)
                    .unwrap_or_default();

                let user_style = entry_square
                    .inner()
                    .as_ref()
                    .map(|sq| sq.style())
                    .unwrap_or_default();

                puzzle_style | user_style
            })
            .collect();
        let gext = Grid::from_vec(gext, squares.cols())
            .expect("Constructing GEXT from valid squares and entries");

        extras.gext = Some(gext);

        Ok(extras)
    }

    fn read_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: Extras,
    ) -> read::Result<(Self, Self::State)> {
        // Build the puzzle with owned data
        let (squares, state) = read_state(&grids, &extras);

        let clues = read_clues(&squares, &strings)?;
        let meta = read_metadata(&header, &strings);

        let crossword = Crossword::new(squares, clues, meta);
        Ok((crossword, state))
    }
}

fn read_state(grids: &Grids, extras: &Extras) -> (Squares, CrosswordState) {
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

                            Cell::new_with_style(solution, style)
                        }
                    };

                    Square::new(cell)
                }
            };

            let entry = match windows_1252_to_char(state) {
                NON_PLAYABLE_CHAR => Square::new_empty(),
                letter => {
                    let mut entry = Entry::new_styled(style);

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
    let solution = squares.map_ref(|square| square.map_ref(|cell| Some(cell.solution.clone())));
    let squares = Squares::new(squares);

    let entries = Grid::from_vec(entries, cols).expect("Read correct lenght entries");

    let timer = extras.ltim.unwrap_or_default();
    let state = CrosswordState::new(solution, entries, timer);

    (squares, state)
}

fn read_clues(squares: &Squares, strings: &Strings) -> read::Result<Clues> {
    let mut entries = BTreeMap::new();

    let mut num: u8 = 1;
    let mut clues_iter = strings.clues.iter().enumerate();

    let mut start_at_pos = |num: u8, start: Position, direction: Direction| -> bool {
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
                clue: clue.to_string(),
            },
            context: "Clues".to_string(),
        });
    }

    Ok(Clues::new(entries))
}

#[cfg(all(test, feature = "puz"))]
mod tests {
    use crate::Crossword;
    use puzzled_io::puz::{PuzReader, read};
    use rstest::rstest;
    use std::fs::File;
    use std::path::PathBuf;

    fn parse_puz(path: PathBuf, strict: bool) -> read::Result<(Crossword, Vec<read::Warning>)> {
        let mut file = File::open(path).expect("puzzle file exists");
        let parser = PuzReader::new(strict);

        parser.read_with_warnings(&mut file)
    }

    #[rstest]
    fn parse_ok_puz(#[files("puzzles/ok/*.puz")] path: PathBuf) {
        let result = parse_puz(path, false);
        let (puzzle, _) = result.expect("puzzle is parsed correctly");

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
        let (_, warnings) = result.expect("puzzle is parsed correctly");

        assert!(!warnings.is_empty());
    }
}
