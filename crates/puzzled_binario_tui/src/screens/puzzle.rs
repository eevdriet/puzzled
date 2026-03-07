use derive_more::{Deref, DerefMut};
use puzzled_binario::{Binario, BinarioState, Bit};
use puzzled_core::{Position, SolutionEntry};
use puzzled_tui::{
    Action, ActionResolver, CellRender, CommandHistory, GridRenderState, GridWidget, HandleAction,
    StatefulScreen, TextBlock,
};

use crate::{AppState, BinarioAction};

use ratatui::{
    layout::HorizontalAlignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Deref, DerefMut)]
pub struct RenderBit<'a>(pub SolutionEntry<'a, Bit>);

impl<'a> CellRender<GridRenderState> for RenderBit<'a> {
    fn render_cell(&self, pos: Position, state: &GridRenderState) -> impl Widget {
        // Determine the cell style
        let mut style = Style::default();

        if self.entry.entry().is_none() && self.solution.is_none() {
            style = style.fg(Color::DarkGray).dim();
        } else {
            style = style.fg(Color::White);
        }

        if pos == state.cursor {
            style = style.add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK);
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .border_type(BorderType::Rounded)
            .title_alignment(HorizontalAlignment::Center)
            .title_style(style);

        let symbol = match pos == state.cursor {
            true => "EE".to_string(),
            false => self.to_string(),
        };

        let text = Text::from(symbol).style(style);

        TextBlock {
            text,
            block,
            h_align: state.options.h_align,
            v_align: state.options.v_align,
        }
    }
}

pub struct PuzzleScreen {
    puzzle: Binario,
    solve_state: BinarioState,
    render_state: GridRenderState,

    commands: CommandHistory<BinarioState>,
}

impl PuzzleScreen {
    pub fn new(puzzle: Binario, solve_state: BinarioState, render_state: GridRenderState) -> Self {
        Self {
            puzzle,
            solve_state,
            render_state,

            commands: CommandHistory::default(),
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
        match action {
            Action::Cancel => resolver.prev_screen(),
            Action::Quit => resolver.exit(),

            action => {
                let mut widget = GridWidget(&self.solve_state.to_merged());
                widget.on_action(action, resolver, &mut self.render_state);
            }
        }
    }
}
