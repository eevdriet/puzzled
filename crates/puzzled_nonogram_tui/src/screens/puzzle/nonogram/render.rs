use puzzled_core::{Entry, Position};
use puzzled_nonogram::{Colors, Fill};
use puzzled_tui::{AppContext, AsApp, CellRender, GridRenderState, ThemeStyled};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Widget,
    text::{Line, Text},
};

use crate::NonogramApp;

#[derive(Debug)]
pub struct RenderFill<'a> {
    pub fill: &'a Fill,
}

impl<'a> ThemeStyled for RenderFill<'a> {}

#[derive(Debug, Clone, Copy)]
pub struct RenderFillState<'a> {
    pub colors: &'a Colors,
}

impl<'a> CellRender<NonogramApp, &Colors> for Entry<RenderFill<'a>> {
    fn render_cell(
        &self,
        pos: Position,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<NonogramApp>,
        state: &GridRenderState,
        colors: &&Colors,
    ) {
        let mut style = self.theme_style(&ctx.theme);
        style = style.fg(ctx.theme.palette.light2);

        if let Some(render) = self.entry()
            && let Some(color) = colors.get(render.fill)
        {
            style = style.fg(color.as_app());
        }

        if state.selection.contains(pos, area) {
            style = style.patch(ctx.theme.selection);
        }

        let symbol = if state.cursor == pos {
            'E'
        } else {
            match self.entry() {
                Some(fill) => fill.fill.symbol(),
                _ => '◌',
            }
        }
        .to_string();

        // Render
        let text = vec![
            Line::from(symbol.repeat(state.options.cell_width as usize));
            state.options.cell_height as usize
        ];
        Text::from(text).style(style).render(area, buf);
    }
}
