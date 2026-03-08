use derive_more::{Deref, DerefMut};
use puzzled_core::{Entry, Position, Puzzle, Square, SquareGridRef};
use puzzled_crossword::{ClueDirection, Crossword, Solution};
use puzzled_tui::{
    Action, ActionResolver, CellRender, HandleAction, RenderGrid, RenderSize, TextBlock, align_area,
};

use ratatui::{
    layout::HorizontalAlignment,
    prelude::{Buffer, Rect, Size},
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
        let grid = state.solve.entries.map_ref(RenderSquareSolution);
        let title = Crossword::title(state.puzzle.meta());

        let block = Block::default()
            .title(format!(" {title} "))
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

impl RenderSize for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render_size(&self, state: &Self::State) -> Size {
        let mut size = state.puzzle.squares().render_size(&state.render);

        // Border around puzzle grid
        size.width += 2;
        size.height += 2;

        size
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
        let mut grid = SquareGridRef(&state.solve.entries);
        grid.on_action(action, resolver, &mut state.render);
    }
}

#[derive(Deref, DerefMut)]
struct RenderSquareSolution<'a>(pub(crate) &'a Square<Entry<Solution>>);

impl<'a> CellRender<PuzzleScreenState> for RenderSquareSolution<'a> {
    fn render_cell(&self, pos: Position, state: &PuzzleScreenState) -> impl Widget {
        let cursor = state.render.cursor;

        // Determine the styles
        let base_style = Style::default();

        let mut border_style = base_style;
        let mut clue_style = base_style;
        let mut entry_style = base_style;

        // Playable v.s. non-playable cells
        match self.0.as_ref().is_some() {
            true => {
                border_style = base_style.fg(Color::DarkGray);
                clue_style = base_style.fg(Color::White).dim();
            }
            false => {
                border_style = base_style.fg(Color::Black).dim();
            }
        }

        if let Some((across, down)) = state.puzzle.clues().get_clues(cursor) {
            tracing::info!("Position {pos} clues: A: {across:?} | D: {down:?}");
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

        if pos == cursor {
            border_style = base_style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded);

        if let Some(num) = state.puzzle.clues().get_num(pos) {
            block = block
                .title(num.to_string())
                .title_style(clue_style)
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

        let text = Text::from(symbol).style(base_style.fg(Color::White));

        TextBlock {
            text,
            block,
            h_align: state.render.options.h_align,
            v_align: state.render.options.v_align,
        }
    }
}
