use puzzled_nonogram::{Fill, Nonogram, NonogramState};
use puzzled_tui::{ActionHistory, FocusManager, SidedGridRenderState};
use ratatui::layout::Size;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Focus {
    #[default]
    Nonogram,

    RowRules,
    ColRules,
}

pub struct PuzzleScreenState {
    // Nonogram state
    pub puzzle: Nonogram,
    pub solve: NonogramState,
    pub render: SidedGridRenderState,

    pub fill: Fill,
    pub focus: FocusManager<Focus>,

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
