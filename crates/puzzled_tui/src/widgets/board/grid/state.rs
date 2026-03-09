use puzzled_core::{Direction, Position};
use ratatui::layout::{Offset, Position as AppPosition, Rect};
use tui_scrollview::ScrollViewState;

use crate::GridOptions;

#[derive(Debug, Default)]
pub struct GridRenderState {
    pub options: GridOptions,

    /// Visible area of the grid
    pub viewport: Rect,

    /// Offset of the grid with its top-left most cell
    pub offset: Position,
    pub cursor: Position,
    pub direction: Direction,
    pub scroll: ScrollViewState,
}

impl GridRenderState {
    pub fn to_grid(&self, app_pos: AppPosition) -> Option<Position> {
        tracing::info!("App {app_pos} to grid");

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

        // Remove inner cell borders if set
        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        if let Some(inner) = opts.inner {
            let block_w = inner.width * cell_w + 1;
            let block_h = inner.height * cell_h + 1;

            let h_border_count = x / block_w;
            let v_border_count = y / block_h;

            x -= h_border_count;
            y -= v_border_count;
        }

        // Adjust for variable cell size
        let mut row = usize::from(y / cell_h);
        let mut col = usize::from(x / cell_w);

        // Translate with the current scroll
        col += self.cursor.col;
        row += self.cursor.row;

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
        if let Some(inner) = opts.inner {
            x += col / inner.width;
            y += row / inner.height;
        }

        let app_pos = AppPosition::new(x, y);
        vp.contains(app_pos).then_some(app_pos)
    }

    pub fn ensure_cursor_visible(&mut self) {
        let cell_w = self.options.cell_width;
        let cell_h = self.options.cell_height;
        let vp = self.viewport;

        // Determine the cursor and current offset position (viewport aligned)
        let cursor_x = self.cursor.col as u16 * cell_w;
        let cursor_y = self.cursor.row as u16 * cell_h;

        let mut offset_x = self.scroll.offset().x;
        let mut offset_y = self.scroll.offset().y;

        // Horizontal
        if offset_x > cursor_x {
            offset_x = cursor_x;
        } else if offset_x + vp.width < cursor_x + cell_w {
            offset_x = cursor_x + cell_w - vp.width;
        }

        // Vertical
        if offset_y > cursor_y {
            offset_y = cursor_y;
        } else if offset_y + vp.height < cursor_y + cell_w {
            offset_y = cursor_y + cell_w - vp.height;
        }

        let offset = AppPosition::new(offset_x, offset_y);

        self.scroll.set_offset(offset);
    }
}
