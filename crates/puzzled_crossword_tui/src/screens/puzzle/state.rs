use puzzled_core::Position;
use puzzled_crossword::{ClueDirection, ClueId, Crossword, CrosswordState};
use puzzled_tui::{FocusManager, GridRenderState};
use ratatui::widgets::ListState;

use crate::Focus;

pub struct PuzzleScreenState {
    // Crossword state
    pub puzzle: Crossword,
    pub solve: CrosswordState,
    pub render: GridRenderState,

    // Clues state
    pub across: ListState,
    pub down: ListState,

    // UI state
    pub focus: FocusManager<Focus>,
}

impl PuzzleScreenState {
    pub fn list(&mut self, clue_dir: ClueDirection) -> &mut ListState {
        match clue_dir {
            ClueDirection::Across => &mut self.across,
            ClueDirection::Down => &mut self.down,
        }
    }

    pub fn update_clues_from_cursor(&mut self) {
        let clues = &self.puzzle.clues();
        let clue_dir = ClueDirection::from(self.render.direction);

        // Determine the clues under the cursor
        if let Some((across, down)) = clues.get_clues(self.render.cursor) {
            // Derive the list state and identifiers of the active clue
            let nums: Vec<_> = clues
                .iter_direction(clue_dir)
                .map(|clue| clue.num())
                .collect();

            let (list, num) = match clue_dir {
                ClueDirection::Across => (&mut self.across, across.num()),
                ClueDirection::Down => (&mut self.down, down.num()),
            };

            // Search for the index of the active clue
            // Note that iter_direction above is ordered, so a binary search is valid
            if let Ok(idx) = nums.binary_search(&num) {
                list.select(Some(idx));
            }
        }

        self.render.ensure_cursor_visible();
    }

    pub fn update_cursor_from_clues(&mut self) {
        // Determine the currently selected clue
        let clues = &self.puzzle.clues();
        let direction = ClueDirection::from(self.render.direction);
        let selected = match direction {
            ClueDirection::Across => self.across.selected(),
            ClueDirection::Down => self.down.selected(),
        };

        let Some(idx) = selected else {
            return;
        };

        let Some(clue) = clues.iter_direction(direction).nth(idx) else {
            return;
        };

        // Then update the cursor to be its starting position
        self.render.cursor = clue.start();
        self.render.ensure_cursor_visible();
    }
}
