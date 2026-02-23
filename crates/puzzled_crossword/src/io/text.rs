use puzzled_core::Cell;
use puzzled_io::{
    format,
    text::{
        TxtPuzzle,
        read::{self, TxtState},
    },
};

use crate::{ClueSpec, Crossword, CrosswordCell, Direction, Solution, Square, Squares};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid clue specification: {reason}")]
    InvalidClueSpec { reason: String },
}

impl TxtPuzzle for Crossword {
    fn from_text(reader: &mut TxtState) -> read::Result<Self> {
        // Read in the squares grid
        let mut read_square = |token: &str| -> Square {
            match token {
                "." => None,
                word => {
                    let solution = Solution::from(word);
                    let cell = Cell::new(solution);

                    Some(CrosswordCell::new(cell))
                }
            }
        };

        let squares = reader.read_grid(&mut read_square)?;
        let squares = Squares::new(squares);

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
            "A" => Direction::Across,
            "D" => Direction::Down,
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
