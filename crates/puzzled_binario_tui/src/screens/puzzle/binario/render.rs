use puzzled_binario::{Bit, LineBits};
use puzzled_core::{Algerbraic, Entry, Position, Size};
use puzzled_tui::{
    AppContext, CellRender, EdgeRender, GridRenderState, SidedGridRenderState, TextBlock, Theme,
    ThemeStyled, center_area,
};
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size as AppSize},
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
    pub fn count_style(&self, count: isize, line_len: usize, theme: &Theme) -> Style {
        match count {
            // Valid count
            count if count > 0 && count <= (line_len / 2) as isize => self.theme_style(theme),

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

impl EdgeRender<BinarioApp, Size> for bool {
    fn render_row(
        &self,
        row: usize,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        state: &Size,
    ) {
        let style = if *self {
            Style::default()
        } else {
            ctx.theme.incorrect
        };
        let width = state.rows.to_string().len() as u16;
        let area = center_area(area, AppSize::new(width, 1));

        Text::styled((row + 1).to_string(), style).render(area, buf);
    }
    fn render_col(
        &self,
        col: usize,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        state: &Size,
    ) {
        let style = if *self {
            Style::default()
        } else {
            ctx.theme.incorrect
        };
        let width = state.cols.to_algebraic().len() as u16;
        let area = center_area(area, AppSize::new(width, 1));

        Text::styled(col.to_algebraic(), style).render(area, buf);
    }

    fn height(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        _state: &Size,
    ) -> u16 {
        1
    }

    fn width(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        state: &Size,
    ) -> u16 {
        state.rows.to_string().len() as u16
    }
}

impl EdgeRender<BinarioApp, Size> for LineBits {
    fn render_row(
        &self,
        _row: usize,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<BinarioApp>,
        render: &SidedGridRenderState,
        state: &Size,
    ) {
        let zero_style =
            RenderBit { bit: &Bit::Zero }.count_style(self.zeroes, state.cols, &ctx.theme);
        let one_style = RenderBit { bit: &Bit::One }.count_style(self.ones, state.cols, &ctx.theme);
        let area = center_area(area, AppSize::new(self.width(area, ctx, render, state), 1));

        Line::default()
            .spans(vec![
                Span::styled(self.zeroes.to_string(), zero_style),
                Span::raw(" "),
                Span::styled(self.ones.to_string(), one_style),
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
        state: &Size,
    ) {
        let zeroes = self.zeroes.to_string();
        let ones = self.ones.to_string();
        let width = zeroes.len().max(ones.len()) as u16;
        let area = center_area(
            area,
            AppSize::new(width, self.height(area, ctx, render, state)),
        );

        let zero_style =
            RenderBit { bit: &Bit::Zero }.count_style(self.zeroes, state.rows, &ctx.theme);
        let one_style = RenderBit { bit: &Bit::One }.count_style(self.ones, state.rows, &ctx.theme);

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
        _state: &Size,
    ) -> u16 {
        2
    }

    fn width(
        &self,
        _area: Rect,
        _ctx: &AppContext<BinarioApp>,
        _render: &SidedGridRenderState,
        state: &Size,
    ) -> u16 {
        (state.rows.to_string().len() + 1 + state.cols.to_string().len()) as u16
    }
}
