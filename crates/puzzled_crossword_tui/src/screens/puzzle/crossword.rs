use derive_more::{Deref, DerefMut};
use puzzled_core::{Entry, Position, Square};
use puzzled_crossword::Solution;
use puzzled_tui::{
    Action, ActionResolver, CellRender, HandleAction, RenderGrid, RenderSize, TextBlock, align_area,
};

use ratatui::{
    layout::HorizontalAlignment,
    prelude::{Buffer, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, StatefulWidgetRef, Widget},
};

use crate::{AppState, CrosswordAction, PuzzleScreenState};

pub struct CrosswordWidget;

impl StatefulWidgetRef for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let r_state = &state.render;
        let title = state.puzzle.meta().title().unwrap_or("ccrossword");
        let grid = state.solve.entries.map_ref(RenderSquareSolution);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .title_alignment(HorizontalAlignment::Center)
            .border_type(BorderType::Rounded);

        let bordered_area = align_area(
            grid.render_size(r_state),
            block.inner(area),
            HorizontalAlignment::Left,
            r_state.options.v_align,
        );

        block.render(area, buf);
        grid.render(bordered_area, buf, r_state, state);
    }
}

impl HandleAction<CrosswordAction, AppState> for CrosswordWidget {
    type State = PuzzleScreenState;

    fn on_action(
        &mut self,
        action: Action<CrosswordAction>,
        resolver: ActionResolver<CrosswordAction, AppState>,
        state: &mut Self::State,
    ) {
        let grid = &mut state.solve.entries;
        grid.on_action(action, resolver, &mut state.render);
    }
}

#[derive(Deref, DerefMut)]
struct RenderSquareSolution<'a>(pub(crate) &'a Square<Entry<Solution>>);

impl<'a> CellRender<PuzzleScreenState> for RenderSquareSolution<'a> {
    fn render_cell(&self, pos: Position, state: &PuzzleScreenState) -> impl Widget {
        let style = Style::default();

        // Determine the cell style
        let mut style = match &self.0.0 {
            Some(_) => style.fg(Color::DarkGray),
            _ => style.fg(Color::Black),
        };

        if pos == state.render.cursor {
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(style)
            .border_type(BorderType::Rounded);

        if let Some(num) = state.puzzle.clues().get_num(pos) {
            block = block
                .title(num.to_string())
                .title_style(Color::White)
                .bold()
                .title_alignment(HorizontalAlignment::Center);
        }

        let symbol = match &self.0.0 {
            Some(entry) => match entry.entry() {
                Some(Solution::Letter(l)) => l.to_string(),
                Some(sol @ Solution::Rebus(_)) => format!("{}…", sol.first_letter()),
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        let text = Text::from(symbol).style(style.fg(Color::White));

        TextBlock {
            text,
            block,
            h_align: state.render.options.h_align,
            v_align: state.render.options.v_align,
        }
    }
}
