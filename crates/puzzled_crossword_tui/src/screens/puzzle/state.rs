use puzzled_core::Direction;
use puzzled_crossword::{Clue, ClueDirection, Crossword, CrosswordState};
use puzzled_tui::{
    ActionHistory, FocusManager, GridRenderState, Keys, KeysTablePopupState, ListRenderState,
    ensure_cells_visible,
};
use ratatui::{layout::Rect, widgets::ListState};

use crate::{CrosswordApp, Focus, PuzzlePopup};

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
    pub popup: Option<PuzzlePopup>,

    pub pause_state: ListState,
    pub pause_keys: Keys<CrosswordApp>,

    pub help_state: KeysTablePopupState,

    // Other
    pub history: ActionHistory<CrosswordState>,
}

impl ListRenderState for PuzzleScreenState {
    fn get(&self) -> ListState {
        self.clue_list(self.clue_dir)
    }

    fn set(&mut self, state: ListState) {
        let list = self.clue_list_mut(self.clue_dir);
        *list = state;
    }
}

impl PuzzleScreenState {
    pub fn clue_list(&self, clue_dir: Option<ClueDirection>) -> ListState {
        match clue_dir {
            Some(ClueDirection::Across) => self.across,
            Some(ClueDirection::Down) => self.down,
            None => self.across_down,
        }
    }

    pub fn clue_list_mut(&mut self, clue_dir: Option<ClueDirection>) -> &mut ListState {
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
        let clues = self.puzzle.clues();

        // Determine the clues under the cursor
        if let Some((across, down)) = clues.get_clues(self.render.cursor) {
            let across_num = across.num();
            let across_nums: Vec<_> = clues.iter_across().map(|clue| clue.num()).collect();

            if let Ok(idx) = across_nums.binary_search(&across_num) {
                self.across.select(Some(idx));
            }

            let down_num = down.num();
            let down_nums: Vec<_> = clues.iter_down().map(|clue| clue.num()).collect();

            if let Ok(idx) = down_nums.binary_search(&down_num) {
                self.down.select(Some(idx));
            }
        }

        // Determine the active clue window based on clue direction
        if self.clue_dir.is_some() {
            let clue_dir = ClueDirection::from(self.render.direction);
            self.clue_dir = Some(clue_dir);
        }

        self.ensure_curr_clue_visible();
    }

    pub fn update_cursor_from_clues(&mut self) {
        // Determine the currently selected clue
        let Some(idx) = self.clue_list_mut(self.clue_dir).selected() else {
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

        self.ensure_curr_clue_visible();
    }

    pub fn ensure_curr_clue_visible(&mut self) {
        let cursor = self.render.cursor;
        let dir = ClueDirection::from(self.render.direction);

        if let Some(clue) = self.puzzle.clues().get_clue(cursor, dir)
            && let Some(end) = clue.positions().last()
        {
            let start = clue.start();
            let cells = Rect {
                x: start.col.min(end.col) as u16,
                y: start.row.min(end.row) as u16,
                width: 1 + start.col.abs_diff(end.col) as u16,
                height: 1 + start.row.abs_diff(end.row) as u16,
            };

            ensure_cells_visible(
                cells,
                self.render.options,
                self.render.viewport,
                &mut self.render.scroll,
            );
        }
    }
}
