use puzzled_binario::Bit;
use puzzled_core::{Position, SolutionEntry};
use puzzled_tui::{CellRender, GridRenderState, LineRender, TextBlock};
use ratatui::{
    layout::HorizontalAlignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct RenderBit<'a> {
    pub solution_entry: &'a SolutionEntry<'a, Bit>,
    pub validity: bool,
}

pub struct RenderBitState<'a> {
    pub render: &'a GridRenderState,
}

pub struct RenderEdgeState;

impl<'a> CellRender<RenderBitState<'a>> for RenderBit<'a> {
    fn render_cell(&self, pos: Position, state: &RenderBitState) -> impl Widget {
        // Determine the cell style
        let entry = self.solution_entry.get();
        let base = Style::default().fg(Color::DarkGray);

        let mut style = match entry {
            None => base.dim(),
            Some(Bit::Zero) => base.fg(Color::Blue),
            Some(Bit::One) => base.fg(Color::White),
        };

        let cursor = state.render.cursor;

        if pos == cursor {
            style = style.bold().not_dim().fg(Color::Yellow);
        }

        if let Some(app_pos) = state.render.to_app(pos)
            && state
                .render
                .selection
                .contains(app_pos, state.render.viewport)
        {
            style = style.fg(Color::Green);
        }

        if entry.is_some() && !self.validity {
            style = style.fg(Color::Red);
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .border_type(BorderType::Rounded)
            .title_alignment(HorizontalAlignment::Center)
            .title_style(style);

        let symbol = match entry {
            Some(entry) => entry.to_string(),
            _ => "".to_string(),
        };

        let text = Text::from(symbol).style(style);

        TextBlock {
            text,
            block,
            h_align: state.render.options.h_align,
            v_align: state.render.options.v_align,
        }
    }
}

impl LineRender<RenderEdgeState> for bool {
    fn render_row(&self, _row: usize, _state: &RenderEdgeState) -> Text<'_> {
        Text::from("R")
    }
    fn render_col(&self, _col: usize, _state: &RenderEdgeState) -> Text<'_> {
        Text::from("C")
    }
}
