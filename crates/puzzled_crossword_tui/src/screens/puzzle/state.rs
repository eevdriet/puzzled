use puzzled_core::Direction;
use puzzled_crossword::{Clue, ClueDirection, Crossword, CrosswordState};
use puzzled_tui::{ActionHistory, FocusManager, GridRenderState};
use ratatui::widgets::ListState;

use crate::Focus;

pub struct PuzzleScreenState {
    // Crossword state
    pub puzzle: Crossword,
    pub solve: CrosswordState,
    pub render: GridRenderState,

    // Clues state
    pub clue_dir: Option<ClueDirection>,
    pub across_down: ListState,
    pub across: ListState,
    pub down: ListState,

    // UI state
    pub focus: FocusManager<Focus>,

    // Other
    pub history: ActionHistory<CrosswordState>,
}

impl PuzzleScreenState {
    pub fn clue_list(&mut self, clue_dir: Option<ClueDirection>) -> &mut ListState {
        match clue_dir {
            Some(ClueDirection::Across) => &mut self.across,
            Some(ClueDirection::Down) => &mut self.down,
            None => &mut self.across_down,
        }
    }

    pub fn clues(&self, clue_dir: Option<ClueDirection>) -> impl Iterator<Item = &Clue> {
        self.puzzle
            .clues()
            .values()
            .filter(move |clue| clue_dir.is_none_or(|dir| clue.direction() == dir))
    }

    pub fn update_clues_from_cursor(&mut self) {
        let clue_dir = ClueDirection::from(self.render.direction);

        // Determine the clues under the cursor
        if let Some(clue) = self.puzzle.clues().get_clue(self.render.cursor, clue_dir) {
            let num = clue.num();
            let nums: Vec<_> = self.clues(self.clue_dir).map(|clue| clue.num()).collect();

            if let Ok(idx) = nums.binary_search(&num) {
                let list = self.clue_list(self.clue_dir);
                list.select(Some(idx));
            }
        }

        self.render.ensure_cursor_visible();
    }

    pub fn update_cursor_from_clues(&mut self) {
        // Determine the currently selected clue
        let Some(idx) = self.clue_list(self.clue_dir).selected() else {
            return;
        };

        let Some(clue) = self.clues(self.clue_dir).nth(idx) else {
            return;
        };

        let clue_direction = clue.direction();
        let cursor = clue.start();

        // Then update the cursor to be its starting position
        self.render.direction = Direction::from(clue_direction);
        self.render.cursor = cursor;
        self.render.ensure_cursor_visible();
    }
}
