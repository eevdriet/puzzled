use derive_more::{Deref, DerefMut};
use puzzled_binario::Bit;
use puzzled_core::{Entry, Position};
use puzzled_tui::{CellRender, GridOptions, TextBlock};
use ratatui::{
    layout::HorizontalAlignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Deref, DerefMut)]
pub struct RenderBit<'a>(pub(crate) &'a Entry<Bit>);

pub struct RenderBitState {
    pub cursor: Position,
    pub opts: GridOptions,
}

impl<'a> CellRender<RenderBitState> for RenderBit<'a> {
    fn render_cell(&self, pos: Position, state: &RenderBitState) -> impl Widget {
        // Determine the cell style
        let mut style = Style::default();

        if self.entry().is_none() {
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
            h_align: state.opts.h_align,
            v_align: state.opts.v_align,
        }
    }
}
