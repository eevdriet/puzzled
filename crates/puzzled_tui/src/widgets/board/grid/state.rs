use puzzled_core::Position;
use ratatui::layout::Position as AppPosition;

use crate::{GridOptions, Viewport};

pub struct GridState {
    pub options: GridOptions,

    /// Visible area of the grid
    pub viewport: Viewport,

    /// Offset of the grid with its top-left most cell
    pub cursor: Position,
}

impl GridState {
    pub fn to_grid(&self, app_pos: AppPosition) -> Option<Position> {
        tracing::info!("App {app_pos} to grid");

        let vp = &self.viewport;
        let opts = &self.options;

        // Ignore positions outside of the viewport
        if !vp.area.contains(app_pos) {
            tracing::debug!("\t Viewport {} does not contain", vp.area);
            return None;
        }

        // Normalize position from the viewport start
        let mut x = app_pos.x.checked_sub(vp.area.x)?;
        let mut y = app_pos.y.checked_sub(vp.area.y)?;

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

        // Ignore positions outside of the viewport
        if !(vp.row_start..vp.row_end).contains(&pos.row) {
            return None;
        }
        if !(vp.col_start..vp.col_end).contains(&pos.col) {
            return None;
        }

        // Normalize position from the viewport start
        let row = (pos.row - vp.row_start) as u16;
        let col = (pos.col - vp.col_start) as u16;

        let mut x = vp.area.x;
        let mut y = vp.area.y;

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

        Some(AppPosition::new(x, y))
    }
}
