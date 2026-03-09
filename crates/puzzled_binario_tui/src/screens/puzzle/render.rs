use derive_more::{Deref, DerefMut};
use puzzled_binario::Bit;
use puzzled_core::{Position, SolutionEntry};
use puzzled_tui::{CellRender, GridRenderState, RenderGrid, RenderSize, TextBlock, align_area};
use ratatui::{
    buffer::Buffer,
    layout::{HorizontalAlignment, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, StatefulWidgetRef, Widget},
};

use crate::{AppState, PuzzleScreen};

impl StatefulWidgetRef for PuzzleScreen {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let r_state = &self.render_state;
        let grid = self.solve_state.to_merged().map(RenderBit);

        let bordered_area = align_area(
            area,
            grid.render_size(&r_state.options),
            r_state.options.h_align,
            r_state.options.v_align,
        );

        grid.render(bordered_area, buf, r_state, r_state);
    }
}

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
