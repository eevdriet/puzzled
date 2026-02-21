use std::time::Instant;

use puzzled_nonogram::{Fill, Nonogram, Order, Position};
use ratatui::layout::{Position as AppPosition, Rect, Size};

use crate::{PuzzleStyle, Selection, Viewport};

#[derive(Debug)]
pub struct PuzzleState {
    pub puzzle: Nonogram,

    pub style: PuzzleStyle,

    pub start_time: Instant,

    /// Selected area of the viewport
    pub selection: Selection,

    /// Position within the puzzle for solving
    pub cursor: AppPosition,

    /// Area used to draw the widget
    pub area: Rect,

    pub viewport: Viewport,

    /// Offset of the puzzle with its top-left most cell
    pub scroll: Position,

    // Solving properties
    pub fill: Fill,

    pub motion_order: Order,
}

impl PuzzleState {
    pub fn new(puzzle: Nonogram, style: PuzzleStyle, fill: Fill) -> Self {
        let order = Order::default();

        Self {
            puzzle,
            style,
            fill,

            selection: Selection::empty(order),
            start_time: Instant::now(),
            cursor: AppPosition::default(),
            area: Rect::default(),
            viewport: Viewport::default(),
            scroll: Position::default(),
            motion_order: order,
        }
    }
    pub fn bounds(&self) -> Rect {
        let width = self.puzzle.cols();
        let height = self.puzzle.rows();

        Rect::new(0, 0, width as u16, height as u16)
    }

    pub fn screen_to_puzzle(&self, area: Rect, screen_pos: AppPosition) -> Option<Position> {
        let puzzle = &self.puzzle;

        // Start from the relative position to the viewport
        let mut x = screen_pos.x.checked_sub(area.x)? as usize;
        let mut y = screen_pos.y.checked_sub(area.y)? as usize;

        tracing::trace!("pos: {screen_pos:?} + viewport: {:?}", area);
        tracing::trace!("pos relative to viewport: {:?}", (x, y));

        let cell_width = self.style.cell_width;
        let cell_height = self.style.cell_height;

        // Remove grid dividors if set
        if let Some(grid_size) = self.style.grid_size {
            let block_w = grid_size * cell_width + 1;
            let block_h = grid_size * cell_height + 1;

            let div_x = x / block_w;
            let div_y = y / block_h;

            x -= div_x;
            y -= div_y;
        }

        // Adjust for variable cell dimensions to find the puzzle position
        let mut col = x / cell_width;
        let mut row = y / cell_height;

        tracing::trace!("col: {col:?} + row: {row:?}");
        tracing::trace!("cols: {:?} + rows: {:?}", puzzle.cols(), puzzle.rows());

        // Translate with the scroll position
        col += self.scroll.col;
        row += self.scroll.row;

        (col < puzzle.cols() && row < puzzle.rows()).then_some(Position { col, row })
    }

    pub fn puzzle_to_screen(&self, puzzle_pos: Position) -> Option<AppPosition> {
        let vp = &self.viewport;

        // Start from the viewport origin
        let mut x = vp.area.x as usize;
        let mut y = vp.area.y as usize;

        // Determine the puzzle position visible within the viewport
        let col = puzzle_pos.col.checked_sub(self.scroll.col)?;
        let row = puzzle_pos.row.checked_sub(self.scroll.row)?;

        // Adjust for variable cell dimensions and add the puzzle position
        let cell_width = self.style.cell_width;
        let cell_height = self.style.cell_height;

        x += col * cell_width;
        y += row * cell_height;

        // Add grid dividors if set
        if let Some(size) = self.style.grid_size {
            x += col / size;
            y += row / size;
        }

        Some(AppPosition::new(x as u16, y as u16))
    }

    fn visible_cells(&self) -> Size {
        let puzzle = &self.puzzle;
        let vp = &self.viewport;

        let top_left = AppPosition::new(vp.area.x, vp.area.y);
        let bottom_right = AppPosition::new(
            vp.area.x + vp.area.width - 1,
            vp.area.y + vp.area.height - 1,
        );

        let start = self.screen_to_puzzle(vp.area, top_left).unwrap_or_else(|| {
            panic!(
                "Viewport top-left {top_left:?} should be in-bounds ({} rows, {} cols)",
                puzzle.rows(),
                puzzle.cols()
            )
        });

        let end = self
            .screen_to_puzzle(vp.area, bottom_right)
            .unwrap_or_else(|| {
                panic!(
                    "Viewport bottom-right {bottom_right:?} should be in-bounds ({} rows, {} cols)",
                    puzzle.rows(),
                    puzzle.cols()
                )
            });

        Size::new(
            (end.col - start.col + 1) as u16,
            (end.row - start.row + 1) as u16,
        )
    }

    pub fn update_viewport(&mut self) {
        let visible = self.visible_cells();
        let vp = &mut self.viewport;

        let rows = self.puzzle.rows();
        vp.row_start = self.scroll.row as u16;
        vp.row_end = (vp.row_start + visible.height).min(rows as u16);

        tracing::trace!(
            "Row range: {}..{} (with {} visible rows and {rows} puzzle rows)",
            vp.row_start,
            vp.row_end,
            visible.height
        );

        let cols = self.puzzle.cols();
        vp.col_start = self.scroll.col as u16;
        vp.col_end = (vp.col_start + visible.width).min(cols as u16);
        tracing::trace!(
            "Col range: {}..{} (with {} visible cols and {cols} puzzle cols)",
            vp.col_start,
            vp.col_end,
            visible.width
        );
    }

    pub fn keep_cursor_visible(&mut self, cursor: AppPosition) {
        let row = cursor.y as usize;
        let col = cursor.x as usize;
        let grid = self.style.grid_size;

        let vp = &self.viewport;
        let (vis_cols, vis_rows) = (vp.visible_cols() as usize, vp.visible_rows() as usize);

        let scroll = self.scroll;

        tracing::trace!("Keep {cursor:?} visible in ({vp:?}");
        tracing::trace!("\tScroll before: {scroll:?}");

        // Cursor is left of the viewport -> make it the offset
        if col < scroll.col {
            self.scroll.col = col;
        }
        // Cursor is right of the viewport -> bring it into view
        else if col >= scroll.col + vis_cols {
            self.scroll.col = col - vis_cols + 1;
        }

        // Cursor is above the viewport -> make it the offset
        if row < scroll.row {
            self.scroll.row = row;
        }
        // Cursor is below the viewport -> bring it into view
        else if row >= scroll.row + vis_rows {
            self.scroll.row = row - vis_rows + 1;

            if let Some(grid) = grid
                && row.is_multiple_of(grid)
            {
                self.scroll.row += 1;
            }
        }

        self.update_viewport();
        tracing::info!("\tScroll after: {scroll:?}");
    }

    pub fn size(&self) -> Size {
        let cols = self.puzzle.cols();
        let rows = self.puzzle.rows();

        let (col_div_count, row_div_count) = match self.style.grid_size {
            Some(size) => ((cols - 1) / size, (rows - 1) / size),
            _ => (0, 0),
        };

        let width = cols * self.style.cell_width + col_div_count;
        let height = rows * self.style.cell_height + row_div_count;

        // Add on 2 for the borders around
        Size::new(width as u16 + 2, height as u16 + 2)
    }
}
