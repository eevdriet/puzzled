use crossterm::event::KeyCode;
use derive_more::{Deref, DerefMut};
use puzzled_core::{Direction, Entry, Position, Puzzle, Solve, Square, SquareGridRef};
use puzzled_crossword::{ClueDirection, Crossword, Solution};
use puzzled_tui::{
    Action, ActionResolver, AppEvent, CellRender, Command, HandleCommand, HandleEvent, RenderGrid,
    RenderSize, TextBlock,
};

use ratatui::{
    layout::{HorizontalAlignment, Margin},
    prelude::{Buffer, Rect, Size},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, StatefulWidgetRef, Widget},
};

use crate::{AppState, CrosswordAction, Focus, PuzzleScreenState};

pub struct CrosswordWidget;

impl StatefulWidgetRef for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Render the outside block with the puzzle title
        let title = Crossword::title(state.puzzle.meta());

        let border_style = if state.focus.current() == &Focus::Crossword {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let block = Block::default()
            .title(format!(" {title} "))
            .borders(Borders::TOP)
            .border_style(border_style)
            .title_alignment(HorizontalAlignment::Center)
            .border_type(BorderType::Rounded);

        let area = block.inner(root);
        block.render(root, buf);

        // Render the active clue
        let clues = state.puzzle.clues();

        if let Some((across, down)) = clues.get_clues(state.render.cursor) {
            let clue_dir = ClueDirection::from(state.render.direction);
            let clue = match clue_dir {
                ClueDirection::Across => across,
                ClueDirection::Down => down,
            };

            let clue_area = area.inner(Margin::new(1, 0));
            let clue_text = format!("{}{}  {}", clue.num(), clue.direction(), clue.text());

            Text::from(clue_text)
                .style(Style::default().fg(Color::White).bold())
                .render(clue_area, buf);
        }

        tracing::info!("Border area: {root}");
        tracing::info!("Grid area: {area}");

        // Render the squares grid
        state.render.viewport = area;
        let grid_area = area.inner(Margin::new(0, 1));

        let grid = state.solve.entries.map_ref(RenderSquareSolution);
        grid.render(grid_area, buf, &state.render, state);
    }
}

impl RenderSize for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render_size(&self, state: &Self::State) -> Size {
        let mut size = state.puzzle.squares().render_size(&state.render);

        // Border around puzzle grid
        size.width += 2;
        size.height += 2;

        // Current clue
        size.height += 1;

        size
    }
}

impl HandleCommand<CrosswordAction, AppState> for CrosswordWidget {
    type State = PuzzleScreenState;

    fn on_command(
        &mut self,
        command: Command<CrosswordAction>,
        resolver: ActionResolver<CrosswordAction, AppState>,
        state: &mut Self::State,
    ) -> bool {
        let mut grid = SquareGridRef(&state.solve.entries);
        grid.on_command(command, resolver, &mut state.render)
    }
}

impl HandleEvent<CrosswordAction, AppState> for CrosswordWidget {
    type State = PuzzleScreenState;

    fn on_event(
        &mut self,
        event: AppEvent,
        resolver: ActionResolver<CrosswordAction, AppState>,
        state: &mut Self::State,
    ) -> bool {
        let Some(key) = event.key() else {
            return false;
        };

        let pos = state.render.cursor;
        let dir = match state.render.direction {
            Direction::Left | Direction::Right => Direction::Right,
            Direction::Up | Direction::Down => Direction::Down,
        };

        match key.code {
            // Movements
            KeyCode::Char(ch) => {
                let _ = state
                    .solve
                    .enter(&pos, Solution::Letter(ch.to_ascii_uppercase()));

                if let Some(next) = pos + dir
                    && state.puzzle.squares().get_fill(next).is_some()
                {
                    state.render.cursor = next;
                }
            }

            KeyCode::Backspace | KeyCode::Delete => {
                state.solve.clear(&pos);

                if let Some(next) = pos - dir
                    && state.puzzle.squares().get_fill(next).is_some()
                {
                    state.render.cursor = next;
                }
            }

            _ => return false,
        }

        true
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
