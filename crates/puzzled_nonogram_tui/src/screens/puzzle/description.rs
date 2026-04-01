use puzzled_tui::{Action, Description, GridRef, Motion};

use crate::{Focus, PuzzleScreenState};

impl Description<PuzzleScreenState> for Action<()> {
    fn description(&self, state: &PuzzleScreenState) -> Option<String> {
        let description = match (self, state.focus.get()) {
            // General
            (Action::Quit, _) => "Quit the game",
            (action @ Action::ShowHelp, _) => return action.description(&()),
            (Action::Cancel, _) => "Pause the game",

            // Nonogram
            (action @ (Action::Undo | Action::Redo), Focus::Nonogram) => {
                return action.description(&());
            }
            (Action::FocusUp, Focus::Nonogram) => "Focus the column rules",
            (Action::FocusRight, Focus::RowRules) | (Action::FocusDown, Focus::ColRules) => {
                "Focus the nonogram"
            }

            // Other
            _ => return None,
        };

        Some(description.to_string())
    }
}

impl Description<PuzzleScreenState> for Motion<()> {
    fn description(&self, state: &PuzzleScreenState) -> Option<String> {
        let grid = GridRef(state.puzzle.fills());
        let desc_state = ("fill".to_string(), grid);

        match (self, state.focus.get()) {
            // Nonogram
            (motion, Focus::Nonogram) => motion.description(&desc_state),
            (motion, Focus::RowRules) => motion.description(&desc_state),

            // Other
            _ => None,
        }
    }
}
