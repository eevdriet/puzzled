use derive_more::{Deref, DerefMut};
use puzzled_core::{Position, SolutionEntry, Square};
use puzzled_crossword::{ClueDirection, Clues, Solution};
use puzzled_tui::{CellRender, GridRenderState, TextBlock};

use ratatui::{
    layout::HorizontalAlignment,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Deref, DerefMut)]
pub(crate) struct RenderSquareSolution<'a> {
    pub solution_entry: &'a Square<SolutionEntry<'a, Solution>>,
}

pub struct RenderSquareState<'a> {
    pub clues: &'a Clues,
    pub render: &'a GridRenderState,
}

impl<'a> CellRender<RenderSquareState<'a>> for RenderSquareSolution<'a> {
    fn render_cell(&self, pos: Position, state: &RenderSquareState) -> impl Widget {
        // Determine the styles
        let base_style = Style::default();

        let (border_style, clue_style, symbol) = match self.solution_entry.as_ref() {
            Some(entry) => {
                let mut border_style = base_style.fg(Color::DarkGray);
                let mut clue_style = base_style.fg(Color::White).dim();

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
                if entry.entry.is_revealed() {
                    border_style = border_style.fg(Color::Blue);
                } else if entry.entry.is_incorrect() {
                    border_style = border_style.fg(Color::Red);
                }

                if pos == state.render.cursor {
                    border_style = if state.render.mode.is_visual() {
                        base_style.fg(Color::LightGreen)
                    } else {
                        base_style.fg(Color::Yellow)
                    }
                    .add_modifier(Modifier::BOLD);
                } else if let Some(app_pos) = state.render.to_app(pos)
                    && state
                        .render
                        .selection
                        .contains(app_pos, state.render.viewport)
                {
                    border_style = base_style.fg(Color::Green);
                }

                // Display the first letter of the solution
                let symbol = match entry.get() {
                    Some(Solution::Letter(l)) => l.to_string(),
                    Some(sol @ Solution::Rebus(_)) => format!("{}…", sol.first_letter()),
                    None => "".to_string(),
                };

                (border_style, clue_style, symbol)
            }
            None => {
                let border_style = base_style.fg(Color::Black).dim();
                let clue_style = base_style;
                let symbol = "".to_string();

                (border_style, clue_style, symbol)
            }
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

        let text = Text::from(symbol).style(base_style.fg(Color::White));

        // Render
        TextBlock {
            text,
            block,
            h_align: state.render.options.h_align,
            v_align: state.render.options.v_align,
        }
    }
}
