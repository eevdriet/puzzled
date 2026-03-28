use derive_more::{Deref, DerefMut};
use puzzled_core::{Entry, Position, Square};
use puzzled_crossword::{ClueDirection, Clues, Solution};
use puzzled_tui::{AppContext, CellRender, GridRenderState, TextBlock, Theme, ThemeStyled};

use ratatui::{
    layout::HorizontalAlignment,
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::CrosswordApp;

#[derive(Deref, DerefMut)]
pub(crate) struct RenderSolution<'a> {
    pub solution: &'a Solution,
}

impl<'a> ThemeStyled for RenderSolution<'a> {
    fn theme_style(&self, theme: &Theme) -> Style {
        Style::default().fg(theme.palette.dark0)
    }
}

pub struct RenderSquareState<'a> {
    pub clues: &'a Clues,
    pub render: &'a GridRenderState,
}

impl<'a> CellRender<CrosswordApp, RenderSquareState<'a>> for Square<Entry<RenderSolution<'a>>> {
    fn render_cell(
        &self,
        pos: Position,
        state: &RenderSquareState,
        ctx: &AppContext<CrosswordApp>,
    ) -> impl Widget {
        // Determine the base styles
        let palette = &ctx.theme.palette;
        let base = self.theme_style(&ctx.theme);
        let text_style = Style::default().fg(palette.light3);

        let mut border_style = base;
        let clue_style = text_style;

        // Style the cells that cover the clues at the cursor position
        if !state.render.mode.is_visual()
            && let Some((across, down)) = state.clues.get_clues(state.render.cursor)
        {
            let clue_dir = ClueDirection::from(state.render.direction);
            let active_clue_style = border_style.fg(palette.cyan).bold();
            let alt_clue_style = base;

            let (across_style, down_style) = match clue_dir {
                ClueDirection::Across => (active_clue_style, alt_clue_style),
                ClueDirection::Down => (alt_clue_style, active_clue_style),
            };

            if across.positions().any(|clue_pos| clue_pos == pos) {
                border_style = across_style;
            }
            if down.positions().any(|clue_pos| clue_pos == pos) {
                border_style = down_style;
            }
        }

        // Apply general cell styling
        let cell_style = state.render.cell_style(pos, &ctx.theme);
        border_style = border_style.patch(cell_style);

        // Display the first letter of the solution
        let symbol = match self.as_ref().and_then(|sq| sq.entry()) {
            Some(render) => match render.solution {
                Solution::Letter(l) => l.to_string(),
                sol @ Solution::Rebus(_) => format!("{}…", sol.first_letter()),
            },
            None => "".to_string(),
        };

        // Widgets
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded);

        if let Some(num) = state.clues.get_num(pos) {
            block = block
                .title(num.to_string())
                .title_style(clue_style)
                .bold()
                .title_alignment(HorizontalAlignment::Left);
        }

        let text = Text::from(symbol).style(text_style);

        // Render
        TextBlock {
            text,
            block,
            h_align: state.render.options.h_align,
            v_align: state.render.options.v_align,
        }
    }
}
