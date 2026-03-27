use puzzled_binario::Bit;
use puzzled_core::{Entry, Position};
use puzzled_tui::{
    AppContext, CellRender, GridRenderState, LineRender, SidesRenderState, TextBlock, Theme,
    ThemeStyled,
};
use ratatui::{
    layout::HorizontalAlignment,
    style::Style,
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::BinarioApp;

#[derive(Debug)]
pub struct RenderBit<'a> {
    pub bit: &'a Bit,
    pub validity: bool,
}

impl<'a> ThemeStyled for RenderBit<'a> {
    fn theme_style(&self, theme: &Theme) -> Style {
        let base = Style::default();

        let mut style = match self.bit {
            Bit::Zero => base.fg(theme.palette.blue),
            Bit::One => base.fg(theme.palette.light3),
        };

        if !self.validity {
            style = style.patch(theme.incorrect);
        }

        style
    }
}

impl<'a> CellRender<BinarioApp, GridRenderState> for Entry<RenderBit<'a>> {
    fn render_cell(
        &self,
        pos: Position,
        state: &GridRenderState,
        ctx: &AppContext<BinarioApp>,
    ) -> impl Widget {
        let mut style = self.theme_style(&ctx.theme);
        style = style.patch(state.cell_style(pos, &ctx.theme));

        let border_type = if self.is_initially_revealed() {
            BorderType::HeavyDoubleDashed
        } else {
            BorderType::Rounded
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .border_type(border_type)
            .title_alignment(HorizontalAlignment::Center)
            .title_style(style);

        let symbol = match self.entry() {
            Some(entry) => entry.bit.to_string(),
            _ => "".to_string(),
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

impl LineRender<BinarioApp, SidesRenderState> for bool {
    fn render_row(
        &self,
        _row: usize,
        _state: &SidesRenderState,
        _ctx: &AppContext<BinarioApp>,
    ) -> Text<'_> {
        Text::from("R")
    }
    fn render_col(
        &self,
        _col: usize,
        _state: &SidesRenderState,
        _ctx: &AppContext<BinarioApp>,
    ) -> Text<'_> {
        Text::from("C")
    }
}
