use puzzled_tui::{Action, Description, Motion, TextObject};

use crate::{CrosswordAction, CrosswordMotion, CrosswordTextObject, Focus, PuzzleScreenState};

impl Description<PuzzleScreenState> for Action<CrosswordAction> {
    fn description(&self, state: &PuzzleScreenState) -> Option<String> {
        let description = match (self, state.focus.get()) {
            // General
            (Action::Quit, _) => "Quit the game",
            (action @ Action::ShowHelp, _) => return action.description(&()),
            (Action::Cancel, _) => "Pause the game",

            // Crossword
            (action @ (Action::Undo | Action::Redo), Focus::Crossword) => {
                return action.description(&());
            }
            (Action::FocusRight, Focus::Crossword) => "Focus the clues",

            // Clues
            (Action::FocusLeft, Focus::Clues) => "Focus the crossword",
            (Action::Select, Focus::Clues) => "Focus the clue in the crossword",

            // Other
            _ => return None,
        };

        Some(description.to_string())
    }
}

impl Description<PuzzleScreenState> for Motion<CrosswordMotion> {
    fn description(&self, state: &PuzzleScreenState) -> Option<String> {
        let description = match (self, state.focus.get()) {
            // Crossword
            (motion, Focus::Crossword) => return motion.description(state.puzzle.squares()),

            // Clues
            (Motion::ColEnd, Focus::Clues) => "Focus the last clue",
            (Motion::ColStart, Focus::Clues) => "Focus the first clue",
            (Motion::Down, Focus::Clues) => "Focus the next clue",
            (Motion::Left | Motion::Right, Focus::Clues) => "Switch between across and down clues",
            (Motion::Up, Focus::Clues) => "Focus the previous clue",

            // Other
            _ => return None,
        };

        Some(description.to_string())
    }
}

impl Description<PuzzleScreenState> for TextObject<CrosswordTextObject> {
    fn description(&self, _state: &PuzzleScreenState) -> Option<String> {
        self.description(&())
    }
}
