use puzzled_io::format;

use crate::{ClueDirection, ClueSpec};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid clue specification: {reason}")]
    InvalidClueSpec { reason: String },
}

// impl TxtPuzzle<CrosswordState> for Crossword {
//     fn read_text(reader: &mut TxtState) -> read::Result<(Self, CrosswordState)> {
//         let (squares, entries) = reader.read_squares_and_entries()?;
//
//         // Read the clues and metadata
//         let clues = read_clues(reader)?;
//         let (metadata, timer) = reader.read_metadata(None)?;
//
//         // Create the state
//         let solutions = squares.map_ref(|square| square.map_ref(|sq| Some(sq.solution.clone())));
//
//         let timer = timer.unwrap_or_default();
//         let state = CrosswordState::new(solutions, entries, timer);
//
//         // Create the puzzle
//         let mut puzzle = Crossword::from_squares(squares, metadata);
//         puzzle.insert_clues(clues);
//
//         Ok((puzzle, state))
//     }
// }

// fn read_clues(reader: &mut TxtState) -> read::Result<Vec<ClueSpec>> {
//     let mut clues = Vec::new();
//
//     let err = |reason: &str| {
//         let error = Error::InvalidClueSpec {
//             reason: reason.to_string(),
//         };
//         format::Error::PuzzleSpecific(Box::new(error))
//     };
//
//     while let Some(line) = reader.next_prefixed("-") {
//         // Validate direction/text separation
//         let (dir_str, text) = line
//             .split_once(':')
//             .ok_or(err("Clues should be specified as <dir> : <text>"))?;
//
//         // Validate the direction of the clue
//         let direction = match dir_str.trim() {
//             "A" => ClueDirection::Across,
//             "D" => ClueDirection::Down,
//             _ => {
//                 return Err(err("Clue direction should be either A (across) or D (down)").into());
//             }
//         };
//
//         // Validate the clue text
//         let text = reader.read_string(text)?;
//
//         // Add the clue
//         let clue = ClueSpec::new(direction, text);
//         clues.push(clue);
//     }
//
//     Ok(clues)
// }
