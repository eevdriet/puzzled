use derive_more::{Deref, DerefMut};
use puzzled_binario::{Binario, Bit};
use puzzled_core::{Position, SolutionEntry};
use puzzled_tui::{CellRender, GridRenderState, TextBlock};
use ratatui::{
    layout::HorizontalAlignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Deref, DerefMut)]
pub struct RenderBit<'a>(pub(crate) SolutionEntry<'a, Bit>);

pub struct RenderBitState<'a> {
    pub puzzle: &'a Binario,
    pub render: &'a GridRenderState,
}

impl<'a> CellRender<RenderBitState<'a>> for RenderBit<'a> {
    fn render_cell(&self, pos: Position, state: &RenderBitState) -> impl Widget {
        // Determine the cell style
        let entry = self.get();
        let base = Style::default().fg(Color::Gray);

        let mut style = match entry {
            None => base.dim(),
            Some(Bit::Zero) => base.fg(Color::White),
            Some(Bit::One) => base.fg(Color::Yellow),
        };

        let size = state.puzzle.cells().size();
        let cursor = state.render.cursor;

        if pos == cursor {
            style = style.add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK);
        }

        if let Some(app_pos) = state.render.to_app(pos)
            && state.render.selection.contains(app_pos, &size)
        {
            style = style.fg(Color::Green);
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .border_type(BorderType::Rounded)
            .title_alignment(HorizontalAlignment::Center)
            .title_style(style);

        let symbol = match entry {
            Some(entry) => entry.to_string(),
            None if pos == cursor => "E".to_string(),
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
