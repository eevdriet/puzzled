use std::ops::Range;

use puzzled_core::{Direction, Position};
use ratatui::layout::{Position as AppPosition, Rect};

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
}
