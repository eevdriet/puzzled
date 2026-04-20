use puzzled_binario::Bit;
use puzzled_core::{Entry, Position};
use puzzled_tui::{
    AppContext, CellRender, EdgeRender, GridRenderState, SidedGridRenderState, TextBlock, Theme,
    ThemeStyled, center_area,
};
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::BinarioApp;

#[derive(Debug)]
pub struct RenderBit<'a> {
    pub bit: &'a Bit,
}

impl<'a> RenderBit<'a> {
    pub fn count_style(&self, count: isize, limit: usize, theme: &Theme) -> Style {
        match count {
            // Valid count
            count if count > 0 && count <= limit as isize => self.theme_style(theme),

            // Zero count
            0 => Style::default().fg(theme.palette.dark2).dim().crossed_out(),

            // Invalid count
            _ => theme.incorrect,
        }
    }
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

impl EdgeRender<BinarioApp, (usize, usize)> for bool {
    fn render_row(
        &self,
        _row: usize,
        area: Rect,
        buf: &mut Buffer,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(usize, usize),
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
        _state: &(usize, usize),
    ) {
        Text::from("C").render(area, buf);
    }

    fn height(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(usize, usize),
    ) -> u16 {
        1
    }

    fn width(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(usize, usize),
    ) -> u16 {
        1
    }
}

impl EdgeRender<BinarioApp, (usize, usize)> for (isize, isize) {
    fn render_row(
        &self,
        _row: usize,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<BinarioApp>,
        render: &SidedGridRenderState,
        state: &(usize, usize),
    ) {
        let zero_style = RenderBit { bit: &Bit::Zero }.count_style(self.0, state.1 / 2, &ctx.theme);
        let one_style = RenderBit { bit: &Bit::One }.count_style(self.1, state.1 / 2, &ctx.theme);
        let area = center_area(area, Size::new(self.width(area, ctx, render, state), 1));

        Line::default()
            .spans(vec![
                Span::styled(self.0.to_string(), zero_style),
                Span::raw(" "),
                Span::styled(self.1.to_string(), one_style),
            ])
            .render(area, buf);
    }
    fn render_col(
        &self,
        _col: usize,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<BinarioApp>,
        render: &SidedGridRenderState,
        state: &(usize, usize),
    ) {
        let zeroes = self.0.to_string();
        let ones = self.1.to_string();
        let width = zeroes.len().max(ones.len()) as u16;
        let area = center_area(
            area,
            Size::new(width, self.height(area, ctx, render, state)),
        );

        let zero_style = RenderBit { bit: &Bit::Zero }.count_style(self.0, state.0 / 2, &ctx.theme);
        let one_style = RenderBit { bit: &Bit::One }.count_style(self.1, state.0 / 2, &ctx.theme);

        Text::from(vec![
            Line::styled(zeroes, zero_style),
            Line::styled(ones, one_style),
        ])
        .render(area, buf);
    }

    fn height(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &(usize, usize),
    ) -> u16 {
        2
    }

    fn width(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        state: &(usize, usize),
    ) -> u16 {
        (state.0.to_string().len() + 1 + state.1.to_string().len()) as u16
    }
}
