use derive_more::{Deref, DerefMut};
use puzzled_core::{Entry, Position, Square};
use puzzled_crossword::{ClueDirection, Clues, Solution};
use puzzled_tui::{CellRender, GridRenderState, TextBlock};

use ratatui::{
    layout::HorizontalAlignment,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Deref, DerefMut)]
pub(crate) struct RenderSquareSolution<'a>(pub(crate) &'a Square<Entry<Solution>>);

pub struct RenderSquareState<'a> {
    pub clues: &'a Clues,
    pub render: &'a GridRenderState,
}

impl<'a> CellRender<RenderSquareState<'a>> for RenderSquareSolution<'a> {
    fn render_cell(&self, pos: Position, state: &RenderSquareState) -> impl Widget {
        // Determine the styles
        let base_style = Style::default();

        let mut clue_style = base_style;

        // Playable v.s. non-playable cells
        let is_playable = self.0.as_ref().is_some();

        let mut border_style = if is_playable {
            clue_style = base_style.fg(Color::White).dim();

            base_style.fg(Color::DarkGray)
        } else {
            base_style.fg(Color::Black).dim()
        };

        if !state.render.mode.is_visual()
            && let Some((across, down)) = state.clues.get_clues(state.render.cursor)
        {
            let clue_dir = ClueDirection::from(state.render.direction);
            let active_clue_style = border_style.fg(Color::Cyan).bold();
            let alt_clue_style = border_style.fg(Color::White).dim();

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

            clue_style = clue_style.not_dim();
        }

        // Cell style
        if let Some(cell) = self.0.as_ref() {
            if cell.is_revealed() {
                border_style = border_style.fg(Color::Blue);
            } else if cell.is_incorrect() {
                border_style = border_style.fg(Color::Red);
            }
        }

        if pos == state.render.cursor {
            border_style = if state.render.mode.is_visual() {
                base_style.fg(Color::LightGreen)
            } else {
                base_style.fg(Color::Yellow)
            }
            .add_modifier(Modifier::BOLD);
        } else if is_playable
            && let Some(app_pos) = state.render.to_app(pos)
            && state
                .render
                .selection
                .contains(app_pos, state.render.viewport)
        {
            border_style = base_style.fg(Color::Green);
        }

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

        let symbol = match &self.0.0 {
            Some(entry) => match entry.entry() {
                Some(Solution::Letter(l)) => l.to_string(),
                Some(sol @ Solution::Rebus(_)) => format!("{}…", sol.first_letter()),
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        let text = Text::from(symbol).style(base_style.fg(Color::White));

        TextBlock {
            text,
            block,
            h_align: state.render.options.h_align,
            v_align: state.render.options.v_align,
        }
    }
}
