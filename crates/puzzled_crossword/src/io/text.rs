use puzzled_core::{Cell, NON_PLAYABLE_CHAR};
use puzzled_io::{
    format,
    text::{
        TxtPuzzle,
        read::{self, TxtState},
    },
};

use crate::{ClueDirection, ClueSpec, Crossword, CrosswordState, Solution, Square};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid clue specification: {reason}")]
    InvalidClueSpec { reason: String },
}

impl TxtPuzzle<CrosswordState> for Crossword {
    fn read_text(reader: &mut TxtState) -> read::Result<Self> {
        // Read in the squares grid
        let mut read_row = |line: &str| {
            line.split_whitespace()
                .map(|token| {
                    if token.len() == 1
                        && token.chars().next().expect("Verified non-zero length")
                            == NON_PLAYABLE_CHAR
                    {
                        Square::new_empty()
                    } else {
                        let solution = Solution::from(token);
                        let cell = Cell::new(Some(solution));

                        Square::new(cell)
                    }
                })
                .collect()
        };

        let squares = reader.read_grid(&mut read_row)?;

        // Read the clues and metadata
        let clues = read_clues(reader)?;
        let metadata = reader.read_metadata(None)?;

        // Create the puzzle
        let mut puzzle = Crossword::from_squares(squares, metadata);
        puzzle.insert_clues(clues);

        Ok(puzzle)
    }
}

fn read_clues(reader: &mut TxtState) -> read::Result<Vec<ClueSpec>> {
    let mut clues = Vec::new();

    let err = |reason: &str| {
        let error = Error::InvalidClueSpec {
            reason: reason.to_string(),
        };
        format::Error::PuzzleSpecific(Box::new(error))
    };

    while let Some(line) = reader.next_prefixed("-") {
        // Validate direction/text separation
        let (dir_str, text) = line
            .split_once(':')
            .ok_or(err("Clues should be specified as <dir> : <text>"))?;

        // Validate the direction of the clue
        let direction = match dir_str.trim() {
            "A" => ClueDirection::Across,
            "D" => ClueDirection::Down,
            _ => {
                return Err(err("Clue direction should be either A (across) or D (down)").into());
            }
        };

        // Validate the clue text
        let text = reader.read_string(text)?;

        // Add the clue
        let clue = ClueSpec::new(direction, text);
        clues.push(clue);
    }

    Ok(clues)
}
