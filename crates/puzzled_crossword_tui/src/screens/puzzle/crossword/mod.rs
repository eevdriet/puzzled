mod clue;
mod commands;
mod render;

pub(crate) use clue::*;
pub(crate) use render::*;

use puzzled_core::Puzzle;
use puzzled_crossword::{ClueDirection, Crossword};
use puzzled_tui::{GridWidget, RenderSize};

use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, StatefulWidgetRef, Widget},
};

use crate::{Focus, PuzzleScreenState};
use tui_scrollview::ScrollView;

pub struct CrosswordWidget;

impl StatefulWidgetRef for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
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
        let grid_size = grid.render_size(&render.options as &_);
        let grid_widget = GridWidget::new(&grid, &cell_state);

        // Render the grid in a scrollable view
        let mut scroll_view = ScrollView::new(grid_size);

        scroll_view.render_stateful_widget(grid_widget, Rect::from(grid_size), render);
        scroll_view.render(grid_area, buf, &mut render.scroll);
    }
}
