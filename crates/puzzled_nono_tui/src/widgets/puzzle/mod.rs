mod actions;
mod state;
mod style;
mod viewport;

pub use actions::*;
pub use state::*;
pub use style::*;
pub use viewport::*;

use nono::Fill;
use ratatui::{
    buffer::Buffer,
    layout::{Position as AppPosition, Rect},
    style::{Color, Modifier, Style},
    widgets::StatefulWidgetRef,
};

use crate::{AppState, Focus, app_to_puzzle, safe_draw_str};

#[derive(Debug, Copy, Clone)]
pub struct PuzzleWidget;

impl StatefulWidgetRef for &PuzzleWidget {
    type State = AppState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        self.draw_puzzle(buf, state);
        self.draw_borders(area, buf, state);
    }
}

impl PuzzleWidget {
    fn draw_puzzle(&self, buf: &mut Buffer, app_state: &AppState) {
        let state = &app_state.puzzle;
        let cell_width = state.style.cell_width as usize;
        let div_style = Style::default().fg(Color::DarkGray);

        // Determine which rows and columns to display
        // Keep track of which positions to draw in the viewport
        let vp = &state.viewport;
        let x_start = vp.area.x;
        let mut y = vp.area.y;

        let cols = state.puzzle.cols();
        let rows = state.puzzle.rows();

        // Keep track of selected positions
        let bounds = state.bounds();
        let range = state.selection.range();
        let selection = range.positions(&bounds);

        for row in vp.row_start..vp.row_end {
            let mut x = x_start;

            for col in vp.col_start..vp.col_end {
                let pos = AppPosition::new(col, row);
                let fill = state.puzzle[app_to_puzzle(pos)];
                let is_selected = selection.contains(&pos);
                let style = PuzzleWidget::cell_style(&fill, pos, is_selected, app_state);

                // Draw cell
                let repeat = state.style.cell_width as usize;
                let symbol = match pos == state.cursor {
                    true => 'E',
                    false => fill.symbol(),
                }
                .to_string()
                .repeat(repeat);

                safe_draw_str(buf, (x, y).into(), symbol, style);

                x += state.style.cell_width;

                // Draw vertical divider
                if let Some(size) = state.style.grid_size
                    && col != cols - 1
                    && (col + 1) % size == 0
                {
                    safe_draw_str(buf, (x, y).into(), "│", div_style);
                    x += 1;
                }
            }

            // Draw horizontal divider
            if let Some(size) = state.style.grid_size
                && (row + 1).is_multiple_of(size)
                && row != rows - 1
            {
                let mut div_x = x_start;
                let div_y = y + state.style.cell_height;

                for col in vp.col_start..vp.col_end {
                    let text = "─".repeat(cell_width);
                    safe_draw_str(buf, (div_x, div_y).into(), text, div_style);
                    div_x += cell_width as u16;

                    if col != cols - 1 && (col + 1).is_multiple_of(size) {
                        safe_draw_str(buf, (div_x, div_y).into(), "┼", div_style);
                        div_x += 1;
                    }
                }

                y += 1;
            }

            // Advance y by cell height
            y += state.style.cell_height;
        }
    }

    fn draw_borders(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        let mut style = Style::default().fg(Color::Gray).dim();
        if matches!(state.focus, Focus::Puzzle) {
            style = style.fg(Color::White).not_dim().bold();
        }

        // Corners
        let x_start = area.x;
        let y_start = area.y;
        let x_end = area.right() - 1;
        let y_end = area.bottom() - 1;

        // Top and bottom borders
        for x in x_start..=x_end {
            buf.set_string(x, y_start, "─", style);
        }
        for x in x_start..=x_end {
            buf.set_string(x, y_end, "─", style);
        }

        // Left and right borders
        for y in y_start..=y_end {
            buf.set_string(x_start, y, "│", style);
        }
        for y in y_start..=y_end {
            buf.set_string(x_end, y, "│", style);
        }

        // Corners
        buf.set_string(x_start, y_start, "┌", style);
        buf.set_string(x_end, y_start, "┐", style);
        buf.set_string(x_start, y_end, "└", style);
        buf.set_string(x_end, y_end, "┘", style);
    }

    fn cell_style(fill: &Fill, pos: AppPosition, is_selected: bool, state: &AppState) -> Style {
        let mut style = Style::default();

        // Fill
        style = match fill {
            Fill::Blank => style.fg(Color::DarkGray).add_modifier(Modifier::DIM),
            Fill::Cross => style.fg(Color::Gray),
            Fill::Color(id) => {
                let (r, g, b) = state
                    .puzzle
                    .style
                    .colors
                    .get(*id as usize - 1)
                    .unwrap_or_else(|| panic!("Color for fill {} should be set", id));

                let rcolor = Color::Rgb(*r, *g, *b);
                style.fg(rcolor)
            }
        };

        // Active line
        if matches!(state.focus, Focus::Puzzle) {
            if pos.x == state.puzzle.cursor.x || pos.y == state.puzzle.cursor.y {
                if !matches!(fill, Fill::Color(_)) {
                    style = style.fg(Color::White);
                }

                // Active cell
                if pos == state.puzzle.cursor {
                    style = style.add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                }
            }

            // // Visual selection
            if is_selected {
                style = style.fg(Color::LightCyan).add_modifier(Modifier::BOLD)
            }
        }

        style
    }
}
