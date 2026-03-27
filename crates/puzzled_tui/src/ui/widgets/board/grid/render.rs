use puzzled_core::{Direction, Position};
use ratatui::{
    layout::{Position as AppPosition, Rect},
    style::Style,
};
use tui_scrollview::ScrollViewState;

use crate::{EventMode, GridOptions, MultiSelection, Theme};

#[derive(Debug, Default, Clone)]
pub struct GridRenderState {
    pub options: GridOptions,

    /// Visible area of the grid
    pub viewport: Rect,

    /// Current selection in the grid
    pub selection: MultiSelection,
    pub mode: EventMode,

    /// Offset of the grid with its top-left most cell
    pub cursor: Position,

    /// Current direction the cursor is facing in within the grid
    pub direction: Direction,

    /// Whether to use directed movements in the grid
    pub use_direction: bool,

    pub scroll: ScrollViewState,
}

impl GridRenderState {
    pub fn to_grid(&self, app_pos: AppPosition) -> Option<Position> {
        let vp = &self.viewport;
        let opts = &self.options;

        // Ignore positions outside of the viewport
        if !vp.contains(app_pos) {
            tracing::debug!("\t Viewport {} does not contain", vp);
            return None;
        }

        // Normalize position from the viewport start
        let mut x = app_pos.x.checked_sub(vp.x)?;
        let mut y = app_pos.y.checked_sub(vp.y)?;

        // Translate with the current scroll
        let AppPosition {
            x: x_offset,
            y: y_offset,
        } = self.scroll.offset();
        x += x_offset;
        y += y_offset;

        // Remove inner cell borders if set
        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        if let Some(inner) = opts.inner_borders {
            let block_w = inner.width * cell_w + 1;
            let block_h = inner.height * cell_h + 1;

            let h_border_count = x / block_w;
            let v_border_count = y / block_h;

            x -= h_border_count;
            y -= v_border_count;
        }

        // Adjust for variable cell size
        let row = usize::from(y / cell_h);
        let col = usize::from(x / cell_w);

        tracing::debug!("\t Translated: {row},{col}");
        Some(Position::new(row, col))
    }

    pub fn to_app(&self, pos: Position) -> Option<AppPosition> {
        let vp = &self.viewport;
        let opts = &self.options;

        // Normalize position from the viewport start
        let row = pos.row as u16;
        let col = pos.col as u16;

        let mut x = vp.x;
        let mut y = vp.y;

        // Adjust for variable cell size
        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        x += col * cell_w;
        y += row * cell_h;

        // Add inner cell borders if set
        if let Some(inner) = opts.inner_borders {
            x += col / inner.width;
            y += row / inner.height;
        }

        let app_pos = AppPosition::new(x, y);
        vp.contains(app_pos).then_some(app_pos)
    }

    pub fn cell_style(&self, pos: Position, theme: &Theme) -> Style {
        let mut style = Style::default();

        // Cell is the at the current cursor position
        if pos == self.cursor {
            style = style.patch(theme.cursor);
        }

        // Cell is visually selected
        if let Some(app_pos) = self.to_app(pos)
            && self.selection.contains(app_pos, self.viewport)
        {
            style = style.patch(theme.selection);
        }

        style
    }

    pub fn ensure_cursor_visible(&mut self, cursor: Position) {
        let cells = Rect {
            x: cursor.col as u16,
            y: cursor.row as u16,
            width: 1,
            height: 1,
        };
        ensure_cells_visible(cells, self.options, self.viewport, &mut self.scroll);
    }
}

pub fn ensure_cells_visible(
    cells: Rect,
    opts: GridOptions,
    vp: Rect,
    scroll: &mut ScrollViewState,
) {
    let cell_w = opts.cell_width;
    let cell_h = opts.cell_height;

    // Determine the cursor and current offset position (viewport aligned)
    let start = AppPosition {
        x: cells.x * cell_w,
        y: cells.y * cell_h,
    };
    let end = AppPosition {
        x: cells.right() * cell_w,
        y: cells.bottom() * cell_h,
    };

    let mut offset_x = scroll.offset().x;
    let mut offset_y = scroll.offset().y;

    // Adjust horizontal offset to keep start_x and end_x visible within the viewport
    if offset_x > start.x {
        offset_x = start.x; // Move the viewport to the left if the start is too far right
    } else if offset_x + vp.width < end.x {
        offset_x = end.x - vp.width; // Move the viewport right to show the end
    } else if offset_x + vp.width < start.x + cell_w {
        offset_x = start.x + cell_w - vp.width; // Make sure the start is visible
    }

    // Adjust vertical offset to keep start_y and end_y visible within the viewport
    if offset_y > start.y {
        offset_y = start.y; // Move the viewport up if the start is too far down
    } else if offset_y + vp.height < end.y {
        offset_y = end.y - vp.height; // Move the viewport down to show the end
    } else if offset_y + vp.height < start.y + cell_h {
        offset_y = start.y + cell_h - vp.height; // Make sure the start is visible
    }

    // Adjust offset
    let offset = AppPosition::new(offset_x, offset_y);
    scroll.set_offset(offset);
}
