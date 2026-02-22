use std::collections::BTreeMap;

use puzzled_core::{CellStyle, Grid, Position};
use puzzled_puz::{
    Extras, Grids, Header, MISSING_ENTRY_CELL, NON_PLAYABLE_CELL, Puz, PuzWrite, SizeCheck, Span,
    Strings, build_string, check_size, format,
    read::{self, read_metadata},
    windows_1252_to_char,
};

use crate::{Cell, Clue, Clues, Crossword, Direction, Solution, Squares};

impl SizeCheck for Clues {
    const KIND: &'static str = "Clues";

    fn check_size(&self) -> format::Result<()> {
        let size = self.len();
        let max_size = u16::MAX as usize;

        check_size(Self::KIND, size, max_size)
    }
}

impl Puz for Crossword {
    fn to_header(&self) -> format::Result<Header> {
        let mut header = Header::default();

        // Grids
        let squares = self.squares();
        squares.check_size()?;
        header.width = squares.cols() as u8;
        header.height = squares.rows() as u8;

        // Clues
        let clues = self.clues();
        clues.check_size()?;
        header.clue_count = clues.len() as u16;

        // Metadata
        let version = self.version().map(|v| v.as_bytes()).unwrap_or_default();
        header.version = version;
        header.write_cib();

        Ok(header)
    }

    fn to_grids(&self) -> format::Result<Grids> {
        // Get the squares and check for overflow of their size
        let squares = self.squares();
        squares.check_size()?;

        let width = squares.rows() as u8;
        let height = squares.cols() as u8;

        // Write the individual grids from the squares
        let solution = squares.map_ref(|square| match square {
            Some(cell) => cell.solution().to_string().chars().next().unwrap_or('\0'),
            _ => NON_PLAYABLE_CELL,
        } as u8);

        let state = squares.map_ref(|square| match square {
            Some(cell) => match cell.entry() {
                Some(v) => v.chars().next().unwrap_or(MISSING_ENTRY_CELL),
                None => MISSING_ENTRY_CELL,
            },
            _ => NON_PLAYABLE_CELL,
        } as u8);

        // Construct the result and validate
        let grids = Grids {
            solution,
            state,
            width,
            height,
        };
        grids.validate()?;

        Ok(grids)
    }

    fn to_strings(&self) -> format::Result<Strings> {
        let clues = self.clues();
        clues.check_size()?;

        let mut strings = Strings {
            clues: Vec::with_capacity(clues.len()),
            ..Default::default()
        };

        strings
            .title
            .write_opt_str0(self.title(), 0)
            .expect("Title");
        strings
            .author
            .write_opt_str0(self.author(), 0)
            .expect("Author");
        strings
            .copyright
            .write_opt_str0(self.copyright(), 0)
            .expect("Copyright");

        strings.clues = Vec::with_capacity(clues.len());

        for (idx, clue) in clues.values().enumerate() {
            let num = idx + 1;
            let context = format!("Clue #{num}");
            strings.clues[idx].write_str0(clue.text()).expect(&context);
        }

        strings
            .notes
            .write_opt_str0(self.notes(), 0)
            .expect("Notes");

        Ok(strings)
    }

    fn to_extras(&self) -> format::Result<puzzled_puz::Extras> {
        let squares = self.squares();
        squares.check_size()?;

        let mut extras = Extras::default();

        // GRBS / RTBL
        if squares.iter_fills().any(|cell| cell.is_rebus()) {
            let mut rebuses: BTreeMap<u8, String> = BTreeMap::new();
            let mut num = 0;

            let grbs = squares.map_ref(|square| match square {
                Some(cell) if cell.is_rebus() => {
                    num += 1;
                    rebuses.insert(num, cell.solution().to_string());

                    num
                }
                _ => 0,
            });

            extras.grbs = Some(grbs);
            extras.rtbl = Some(rebuses);
        }

        // LTIM
        extras.ltim = Some(self.timer());

        // GEXT
        if !squares.iter_fills().all(|cell| cell.style().is_empty()) {
            let gext = squares.map_ref(|square| match square {
                Some(cell) => cell.style(),
                _ => CellStyle::default(),
            });
            extras.gext = Some(gext);
        }

        Ok(extras)
    }

    fn from_puz(
        header: Header,
        grids: Grids,
        strings: Strings,
        extras: puzzled_puz::Extras,
    ) -> read::Result<Self> {
        // Build the puzzle with owned data
        let squares = read_squares(&grids, &extras);
        let clues = read_clues(&squares, &strings)?;
        let meta = read_metadata(&header, &strings, &extras);

        Ok(Crossword::new(squares, clues, meta))
    }
}

fn read_squares(grids: &Grids, extras: &Extras) -> Squares {
    let mut cells = Vec::new();
    eprintln!("Extras: {extras:?}");

    for ((pos, &solution), &state) in grids.solution.iter_indexed().zip(grids.state.iter()) {
        let cell = match solution {
            // Non-playable cells are always black
            b'.' => None,

            byte => {
                // Derive the solution based on the rebus information in the extras
                let solution = match extras.get_rebus(pos) {
                    Some(rebus) => Solution::Rebus(rebus.clone()),
                    None => Solution::Letter(windows_1252_to_char(byte)),
                };

                let style = extras.get_style(pos);
                let mut cell = Cell::new_styled(solution, style);

                // Set the given user state for a playable cell
                if state != MISSING_ENTRY_CELL as u8 {
                    let contents = windows_1252_to_char(state).to_string();
                    cell.enter(contents);
                }

                Some(cell)
            }
        };

        cells.push(cell);
    }

    let squares = Grid::from_vec(cells, grids.solution.cols()).expect("Read correct length region");
    Squares::new(squares)
}

fn read_clues(squares: &Squares, strings: &Strings) -> read::Result<Clues> {
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

#[cfg(all(test, feature = "puz"))]
mod tests {
    use crate::Crossword;
    use puzzled_puz::{PuzReader, read};
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
