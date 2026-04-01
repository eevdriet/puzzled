use puzzled_nonogram::{Fill, Nonogram, NonogramState};
use puzzled_tui::{ActionHistory, FocusManager, Keys, KeysTablePopupState, SidedGridRenderState};
use ratatui::{layout::Size, widgets::ListState};

use crate::NonogramApp;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Focus {
    #[default]
    Nonogram,

    RowRules,
    ColRules,
}

#[derive(Debug, Clone, Copy)]
pub enum PuzzlePopup {
    Pause,
    Help,
}

pub struct PuzzleScreenState {
    // Nonogram state
    pub puzzle: Nonogram,
    pub solve: NonogramState,
    pub render: SidedGridRenderState,

    pub fill: Fill,

    // UI state
    pub focus: FocusManager<Focus>,
    pub popup: Option<PuzzlePopup>,

    pub pause_state: ListState,
    pub pause_keys: Keys<NonogramApp>,

    pub help_state: KeysTablePopupState,

    // Commands
    pub history: ActionHistory<NonogramState>,
}

impl PuzzleScreenState {
    pub fn max_rule_size(&self, size: Size) -> Size {
        let opts = &self.render.grid.options;
        let mut max_rule_width = 0;
        let mut max_rule_height = 0;

        for (line, rule) in self.puzzle.rules().iter() {
            let len = rule.display_len();

            if line.is_row() {
                max_rule_width = max_rule_width.max(len);
            } else {
                max_rule_height = max_rule_height.max(len);
            }
        }

        let height = (max_rule_width as f64 / size.width as f64).ceil() as u16;
        let width = (max_rule_height as f64 / size.height as f64).ceil() as u16;

        Size::new(width * opts.cell_width, height * opts.cell_height)
    }
}
