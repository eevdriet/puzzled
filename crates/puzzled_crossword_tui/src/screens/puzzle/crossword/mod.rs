mod clue;
mod render;

pub(crate) use clue::*;
use crossterm::event::KeyCode;
pub(crate) use render::*;

use puzzled_core::{Direction, Puzzle, Solve, SquareGridRef};
use puzzled_crossword::{ClueDirection, Crossword, Solution};
use puzzled_tui::{
    Action, AppCommand, AppResolver, Command, EventMode, GridWidget, HandleBaseAction,
    HandleMotion, HandleOperator, RenderSize, Widget as AppWidget, handle_square_grid_operator,
};

use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Size},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::{CrosswordApp, Focus, GridMotionState, PuzzleScreenState};
use tui_scrollview::ScrollView;

pub struct CrosswordWidget;

impl AppWidget<CrosswordApp> for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let PuzzleScreenState {
            puzzle,
            solve,
            render,
            focus,
            ..
        } = state;

        // Render the outside block with the puzzle title
        let title = Crossword::title(puzzle.meta());

        let border_style = if focus.get() == &Focus::Crossword {
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
        let clues = puzzle.clues();
        let clues_size = CluesSizeWidget { clues }.render_size(area, &());

        let [clue_area, grid_area] = Layout::vertical(vec![
            Constraint::Length(clues_size.height),
            Constraint::Min(0),
        ])
        .areas(area);

        let clue_dir = ClueDirection::from(render.direction);

        if let Some(clue) = clues.get_clue(render.cursor, clue_dir) {
            let mut is_paused = state.popup.is_some();
            ClueWidget { clue }.render(clue_area, buf, &mut is_paused);
        }

        // Set up the squares grid
        render.viewport = grid_area;

        let render_c = render.clone();
        let cell_state = RenderSquareState {
            clues: puzzle.clues(),
            render: &render_c,
        };

        let grid = solve.entries.map_ref(RenderSquareSolution);
        let grid_size = grid.render_size(clue_area, &render.options);
        let grid_widget = GridWidget::new(&grid, &cell_state);

        // Render the grid in a scrollable view
        let mut scroll_view = ScrollView::new(grid_size);

        scroll_view.render_stateful_widget(grid_widget, Rect::from(grid_size), render);
        scroll_view.render(grid_area, buf, &mut render.scroll);
    }

    fn render_size(&self, area: Rect, state: &Self::State) -> Size {
        let mut size = state
            .puzzle
            .squares()
            .render_size(area, &state.render.options);

        // Border around puzzle grid
        size.width += 2;
        size.height += 2;

        // Current clue
        size.height += 2;

        size
    }

    fn on_command(
        &mut self,
        command: AppCommand<CrosswordApp>,
        resolver: AppResolver<CrosswordApp>,
        state: &mut Self::State,
    ) -> bool {
        match command {
            Command::Operator(op) => {
                return handle_square_grid_operator(
                    op,
                    resolver,
                    &state.render,
                    &mut state.solve,
                    &mut state.history,
                );
            }
            Command::Motion { count, motion, op } => {
                {
                    let squares = SquareGridRef(state.puzzle.squares());
                    let mut custom_state = GridMotionState {
                        puzzle: &state.puzzle,
                    };
                    let positions =
                        squares.handle_motion(count, motion, &mut state.render, &mut custom_state);

                    if let Some(op) = op {
                        state
                            .solve
                            .handle_operator(op, positions, &mut state.history);
                    }
                }

                if !state.render.mode.is_visual() {
                    state.update_clues_from_cursor();
                }
            }
            Command::Action { action, .. } => {
                let pos = state.render.cursor;
                let dir = match state.render.direction {
                    Direction::Left | Direction::Right => Direction::Right,
                    Direction::Up | Direction::Down => Direction::Down,
                };

                if !matches!(state.render.mode, EventMode::Insert) {
                    return state
                        .solve
                        .solutions
                        .handle_action(action, &mut state.render, &mut ());
                }

                match action {
                    Action::Literal(KeyCode::Char(letter)) => {
                        let entry = Solution::Letter(letter.to_ascii_uppercase());
                        state.solve.enter(&pos, entry);

                        if let Some(next) = pos + dir
                            && state.puzzle.squares().get_fill(next).is_some()
                        {
                            state.render.cursor = next;
                        }
                    }

                    _ => {
                        return false;
                    }
                }
            }
            _ => return false,
        }

        true
    }
}
