use puzzled_core::{Entry, Position};
use puzzled_nonogram::{Colors, Fill};
use puzzled_tui::{
    AppContext, AsApp, CellRender, GridRenderState, LineRender, SidesRenderState, ThemeStyled,
};
use ratatui::{
    prelude::Widget,
    text::{Line, Text},
};

use crate::NonogramApp;

#[derive(Debug)]
pub struct RenderFill<'a> {
    pub fill: &'a Fill,
}

impl<'a> ThemeStyled for RenderFill<'a> {}

pub struct RenderFillState<'a> {
    pub colors: &'a Colors,
    pub render: &'a GridRenderState,
}

impl<'a> CellRender<NonogramApp, RenderFillState<'a>> for Entry<RenderFill<'a>> {
    fn render_cell(
        &self,
        pos: Position,
        state: &RenderFillState,
        ctx: &AppContext<NonogramApp>,
    ) -> impl Widget {
        let mut style = self.theme_style(&ctx.theme);
        style = style.fg(ctx.theme.palette.light2);

        if let Some(render) = self.entry()
            && let Some(color) = state.colors.get(render.fill)
        {
            style = style.fg(color.as_app());
        }

        let cell_style = state.render.cell_style(pos, &ctx.theme);
        style = style.patch(cell_style);

        let symbol = match self.entry() {
            Some(fill) => fill.fill.symbol(),
            _ => '◌',
        }
        .to_string();

        let text = vec![
            Line::from(symbol.repeat(state.render.options.cell_width as usize));
            state.render.options.cell_height as usize
        ];
        Text::from(text).style(style)
    }
}

impl LineRender<NonogramApp, SidesRenderState> for bool {
    fn render_row(
        &self,
        _row: usize,
        _state: &SidesRenderState,
        _ctx: &AppContext<NonogramApp>,
    ) -> Text<'_> {
        Text::from("R")
    }
    fn render_col(
        &self,
        _col: usize,
        _state: &SidesRenderState,
        _ctx: &AppContext<NonogramApp>,
    ) -> Text<'_> {
        Text::from("C")
    }
}
