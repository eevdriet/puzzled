use derive_more::{Deref, DerefMut};
use puzzled_core::{Direction, Entry, Position, Square};
use puzzled_crossword::{ClueDirection, Clues, Solution, Squares};
use puzzled_tui::{AsApp, CellRender, GridOptions, RenderSize, Selection, TextBlock};

use ratatui::{
    layout::HorizontalAlignment,
    prelude::Size,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::PuzzleScreenState;

use crate::CrosswordWidget;

impl RenderSize<PuzzleScreenState> for CrosswordWidget {
    fn render_size(&self, state: &PuzzleScreenState) -> Size {
        let mut size = state.puzzle.squares().render_size(&state.render.options);

        // Border around puzzle grid
        size.width += 2;
        size.height += 2;

        // Current clue
        size.height += 2;

        size
    }
}

#[derive(Deref, DerefMut)]
pub(crate) struct RenderSquareSolution<'a>(pub(crate) &'a Square<Entry<Solution>>);

pub struct RenderSquareState<'a> {
    pub cursor: Position,
    pub direction: Direction,
    pub selection: Selection,
    pub clues: &'a Clues,
    pub squares: &'a Squares,
    pub opts: GridOptions,
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

        if let Some((across, down)) = state.clues.get_clues(state.cursor) {
            let clue_dir = ClueDirection::from(state.direction);
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

        let size = state.squares.size();

        if pos == state.cursor {
            border_style = base_style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        } else if is_playable && state.selection.range(size).contains(pos.as_app()) {
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
            h_align: state.opts.h_align,
            v_align: state.opts.v_align,
        }
    }
}
