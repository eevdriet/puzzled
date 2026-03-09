mod commands;
mod render;

pub(crate) use render::*;

use puzzled_core::Puzzle;
use puzzled_crossword::{ClueDirection, Crossword};
use puzzled_tui::{GridWidget, RenderSize};

use ratatui::{
    layout::{HorizontalAlignment, Margin},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, StatefulWidget, StatefulWidgetRef, Widget},
};

use crate::{Focus, PuzzleScreenState};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

pub struct CrosswordWidget;

impl StatefulWidgetRef for CrosswordWidget {
    type State = PuzzleScreenState;

    fn render_ref(&self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let PuzzleScreenState {
            puzzle,
            solve,
            render,
            focus,
            ..
        } = state;

        // Render the outside block with the puzzle title
        let title = Crossword::title(puzzle.meta());

        let border_style = if focus.current() == &Focus::Crossword {
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

        if let Some((across, down)) = clues.get_clues(render.cursor) {
            let clue_dir = ClueDirection::from(render.direction);
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

        // Render the squares grid in a scrollable view
        let grid_area = area.inner(Margin::new(0, 1));
        render.viewport = grid_area;

        let grid = state.solve.entries.map_ref(RenderSquareSolution);
        let grid_size = grid.render_size(&render.options as &_);

        let cell_state = RenderSquareState {
            cursor: render.cursor,
            direction: render.direction,
            clues: puzzle.clues(),
            opts: render.options,
        };

        let grid_widget = GridWidget::new(&grid, &cell_state);

        let mut scroll_view =
            ScrollView::new(grid_size).scrollbars_visibility(ScrollbarVisibility::Always);
        scroll_view.render_stateful_widget(grid_widget, area, render);
        scroll_view.render(grid_area, buf, &mut render.scroll);
    }
}
