mod clue;
mod commands;
mod render;

pub(crate) use clue::*;
pub(crate) use render::*;

use puzzled_core::{Direction, Puzzle, Solve, SquareGridRef};
use puzzled_crossword::{ClueDirection, Crossword, Solution};
use puzzled_tui::{
    Action, AppCommand, AppResolver, AsCore, Command, EventMode, GridWidget, HandleBaseMotion,
    HandleOperator, Operator, RenderSize, Widget as AppWidget,
};

use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Size},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::{CrosswordApp, Focus, PuzzleScreenState};
use tui_scrollview::ScrollView;

pub struct CrosswordWidget;

impl AppWidget<CrosswordApp> for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render(&mut self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mode = state.render.mode;

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
        let clues_size = CluesSizeWidget { clues }.render_size(&area);

        let [clue_area, grid_area] = Layout::vertical(vec![
            Constraint::Length(clues_size.height),
            Constraint::Min(0),
        ])
        .areas(area);

        let clue_dir = ClueDirection::from(render.direction);

        if let Some(clue) = clues.get_clue(render.cursor, clue_dir) {
            let mut is_paused = state.is_paused;
            ClueWidget { clue }.render(clue_area, buf, &mut is_paused);
        }

        // Set up the squares grid
        render.viewport = grid_area;

        let cell_state = RenderSquareState {
            cursor: render.cursor,
            direction: render.direction,
            clues: puzzle.clues(),
            squares: puzzle.squares(),
            selection: render.selection,
            opts: render.options,
            mode,
        };

        let grid = solve.entries.map_ref(RenderSquareSolution);
        let grid_size = grid.render_size(&render.options);
        let grid_widget = GridWidget::new(&grid, &cell_state);

        // Render the grid in a scrollable view
        let mut scroll_view = ScrollView::new(grid_size);

        scroll_view.render_stateful_widget(grid_widget, Rect::from(grid_size), render);
        scroll_view.render(grid_area, buf, &mut render.scroll);
    }

    fn render_size(&self, state: &Self::State) -> Size {
        let mut size = state.puzzle.squares().render_size(&state.render.options);

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
                if state.render.mode.is_visual() {
                    let size = state.puzzle.squares().size();
                    let positions = state
                        .render
                        .selection
                        .range(size)
                        .positions()
                        .map(|pos| pos.as_core());

                    state
                        .solve
                        .handle_operator(op, positions, &mut state.history);

                    let mode = match op {
                        Operator::Change => EventMode::Insert,
                        _ => EventMode::Normal,
                    };
                    resolver.set_mode(mode);
                } else if !op.requires_motion() {
                    let positions = vec![state.render.cursor];

                    state
                        .solve
                        .handle_operator(op, positions, &mut state.history);
                } else {
                    return false;
                }
            }
            Command::Motion { count, motion, op } if state.render.mode.is_visual() => {
                tracing::info!("Visual motion: {motion:?}");
                assert!(op.is_none());

                let squares = SquareGridRef(state.puzzle.squares());
                let positions = squares.handle_base_motion(count, motion, &mut state.render);

                if let Some(end) = positions.into_iter().last() {
                    state.render.selection.update(end);
                }
            }
            Command::Motion { count, motion, op } => {
                tracing::info!("Other motion: {motion:?}");

                {
                    let squares = SquareGridRef(state.puzzle.squares());
                    let positions = squares.handle_base_motion(count, motion, &mut state.render);

                    if let Some(op) = op {
                        state
                            .solve
                            .handle_operator(op, positions, &mut state.history);
                    }
                }

                state.update_clues_from_cursor();
            }
            Command::Action { action, .. } => {
                let pos = state.render.cursor;
                let dir = match state.render.direction {
                    Direction::Left | Direction::Right => Direction::Right,
                    Direction::Up | Direction::Down => Direction::Down,
                };

                match action {
                    Action::Insert(letter) => {
                        let entry = Solution::Letter(letter.to_ascii_uppercase());
                        state.solve.enter(&pos, entry);

                        if let Some(next) = pos + dir
                            && state.puzzle.squares().get_fill(next).is_some()
                        {
                            state.render.cursor = next;
                        }
                    }

                    Action::DeleteLeft => {
                        state.solve.clear(&pos);

                        if let Some(next) = pos - dir
                            && state.puzzle.squares().get_fill(next).is_some()
                        {
                            state.render.cursor = next;
                        }
                    }

                    Action::DeleteRight => {
                        state.solve.clear(&pos);
                    }

                    _ => return false,
                }
            }
            _ => return false,
        }

        true
    }
}
