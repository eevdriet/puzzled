use puzzled_binario::Bit;
use puzzled_core::{Entry, Position};
use puzzled_tui::{
    AppContext, CellRender, EdgeRender, GridRenderState, SidedGridRenderState, TextBlock, Theme,
    ThemeStyled,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::BinarioApp;

#[derive(Debug)]
pub struct RenderBit<'a> {
    pub bit: &'a Bit,
}

impl<'a> ThemeStyled for RenderBit<'a> {
    fn theme_style(&self, theme: &Theme) -> Style {
        let base = Style::default();

        match self.bit {
            Bit::Zero => base.fg(theme.palette.blue),
            Bit::One => base.fg(theme.palette.light3),
        }
    }
}

impl<'a> CellRender<BinarioApp, ()> for Entry<RenderBit<'a>> {
    fn render_cell(
        &self,
        pos: Position,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<BinarioApp>,
        render: &GridRenderState,
        _state: &(),
    ) {
        let mut style = self.theme_style(&ctx.theme);

        let cell_style = render.cell_style(pos, &ctx.theme);
        style = style.patch(cell_style);

        let border_type = if self.is_initially_revealed() {
            BorderType::HeavyDoubleDashed
        } else {
            BorderType::Rounded
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .border_type(border_type);

        let symbol = match self.entry() {
            Some(entry) => entry.bit.to_string(),
            _ => "".to_string(),
        };

        let text = Text::from(symbol).style(style);

        TextBlock {
            text,
            block,
            h_align: render.options.h_align,
            v_align: render.options.v_align,
        }
        .render(area, buf);
    }
}

impl EdgeRender<BinarioApp, ()> for bool {
    fn render_row(
        &self,
        _row: usize,
        area: Rect,
        buf: &mut Buffer,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(),
    ) {
        Text::from("R").render(area, buf);
    }
    fn render_col(
        &self,
        _col: usize,
        area: Rect,
        buf: &mut Buffer,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(),
    ) {
        Text::from("C").render(area, buf);
    }

    fn height(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(),
    ) -> u16 {
        1
    }

    fn width(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(),
    ) -> u16 {
        1
    }
}
