use derive_more::{Deref, DerefMut};
use puzzled_binario::{Binario, BinarioState, Bit};
use puzzled_core::{Position, SolutionEntry};
use puzzled_tui::{
    Action, ActionResolver, CellRender, GridRenderState, GridWidget, HandleAction, StatefulScreen,
};

use crate::{AppState, BinarioAction};

use ratatui::{
    style::{Modifier, Style},
    text::Text,
};

const T: Bit = Bit::One;
const F: Bit = Bit::Zero;

#[derive(Deref, DerefMut)]
pub struct RenderBit<'a>(pub SolutionEntry<'a, Bit>);

impl<'a> CellRender<GridRenderState> for RenderBit<'a> {
    fn render_cell(&self, pos: Position, state: &GridRenderState) -> Text<'_> {
        let symbol = match pos == state.cursor {
            true => 'E'.to_string(),
            false => self.to_string(),
        };

        let mut style = Style::default();
        if pos == state.cursor {
            style = style.add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK);
        }

        Text::from(symbol).style(style)
    }
}

pub struct PuzzleScreen {
    puzzle: Binario,
    solve_state: BinarioState,
    render_state: GridRenderState,
}

impl PuzzleScreen {
    pub fn new(puzzle: Binario, solve_state: BinarioState, render_state: GridRenderState) -> Self {
        Self {
            puzzle,
            solve_state,
            render_state,
        }
    }
}

impl StatefulScreen<BinarioAction, AppState> for PuzzleScreen {
    fn render(&mut self, frame: &mut ratatui::Frame, _state: &AppState) {
        let widget = GridWidget(&self.solve_state.to_merged().map(RenderBit));

        frame.render_stateful_widget(widget, frame.area(), &mut self.render_state);
    }

    fn on_action(
        &mut self,
        action: Action<BinarioAction>,
        resolver: ActionResolver<BinarioAction, AppState>,
        _state: &mut AppState,
    ) {
        let mut widget = GridWidget(&self.solve_state.to_merged());
        widget.on_action(action, resolver, &mut self.render_state);
    }
}
