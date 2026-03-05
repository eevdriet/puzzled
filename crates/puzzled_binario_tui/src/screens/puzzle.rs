use puzzled_core::Grid;
use puzzled_tui::{Action, ActionResolver, GridState, GridWidget, HandleAction, StatefulScreen};

use crate::{AppState, BinarioAction, RenderBit};

pub struct PuzzleScreen {
    grid: Grid<RenderBit>,
    state: GridState,
}

impl PuzzleScreen {
    pub fn new(grid: Grid<RenderBit>, state: GridState) -> Self {
        Self { grid, state }
    }
}

impl StatefulScreen<BinarioAction, AppState> for PuzzleScreen {
    fn render(&mut self, frame: &mut ratatui::Frame, _state: &AppState) {
        let widget = GridWidget(&self.grid);

        frame.render_stateful_widget(widget, frame.area(), &mut self.state);
    }

    fn on_action(
        &mut self,
        action: Action<BinarioAction>,
        resolver: ActionResolver<BinarioAction, AppState>,
        _state: &mut AppState,
    ) {
        let mut widget = GridWidget(&self.grid);
        widget.on_action(action, resolver, &mut self.state);
    }
}
