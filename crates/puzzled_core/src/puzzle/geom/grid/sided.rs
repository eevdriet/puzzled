use std::fmt::{self, Display};

use crate::Grid;

#[derive(Debug, thiserror::Error, Clone)]
pub enum SidedGridError {
    #[error("The {side} side has {found} columns, expected {expected}")]
    InvalidColCount {
        side: String,
        found: usize,
        expected: usize,
    },
}

#[derive(Debug)]
pub struct SidedGrid<C, T, R, B, L> {
    pub grid: Grid<C>,

    pub top: Option<Vec<T>>,
    pub right: Option<Vec<R>>,
    pub bottom: Option<Vec<B>>,
    pub left: Option<Vec<L>>,
}

impl<C, T, R, B, L> SidedGrid<C, T, R, B, L> {
    pub fn new(grid: Grid<C>) -> Self {
        SidedGrid {
            grid,
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }

    pub fn with_top(mut self, top: Vec<T>) -> Result<Self, SidedGridError> {
        if top.len() != self.grid.cols() {
            return Err(SidedGridError::InvalidColCount {
                side: "top".to_string(),
                found: top.len(),
                expected: self.grid.cols(),
            });
        }

        self.top = Some(top);
        Ok(self)
    }

    pub fn with_top_value(mut self, val: T) -> Self
    where
        T: Clone,
    {
        self.top = Some(vec![val; self.grid.cols()]);
        self
    }

    pub fn with_bottom(mut self, bottom: Vec<B>) -> Result<Self, SidedGridError> {
        if bottom.len() != self.grid.cols() {
            return Err(SidedGridError::InvalidColCount {
                side: "bottom".to_string(),
                found: bottom.len(),
                expected: self.grid.cols(),
            });
        }

        self.bottom = Some(bottom);
        Ok(self)
    }

    pub fn with_bottom_value(mut self, val: B) -> Self
    where
        B: Clone,
    {
        self.bottom = Some(vec![val; self.grid.cols()]);
        self
    }

    pub fn with_left(mut self, left: Vec<L>) -> Result<Self, SidedGridError> {
        if left.len() != self.grid.rows() {
            return Err(SidedGridError::InvalidColCount {
                side: "left".to_string(),
                found: left.len(),
                expected: self.grid.rows(),
            });
        }

        self.left = Some(left);
        Ok(self)
    }

    pub fn with_left_value(mut self, val: L) -> Self
    where
        L: Clone,
    {
        self.left = Some(vec![val; self.grid.rows()]);
        self
    }

    pub fn with_right(mut self, right: Vec<R>) -> Result<Self, SidedGridError> {
        if right.len() != self.grid.rows() {
            return Err(SidedGridError::InvalidColCount {
                side: "right".to_string(),
                found: right.len(),
                expected: self.grid.rows(),
            });
        }

        self.right = Some(right);
        Ok(self)
    }

    pub fn with_right_value(mut self, val: R) -> Self
    where
        R: Clone,
    {
        self.right = Some(vec![val; self.grid.rows()]);
        self
    }

    pub fn map_grid_ref<'a, U, F>(&'a self, f: F) -> SidedGrid<U, T, R, B, L>
    where
        F: FnMut(&'a C) -> U,
        T: Clone,
        R: Clone,
        B: Clone,
        L: Clone,
    {
        let grid = self.grid.map_ref(f);

        SidedGrid {
            grid,
            top: self.top.clone(),
            right: self.right.clone(),
            bottom: self.bottom.clone(),
            left: self.left.clone(),
        }
    }
}

pub struct SidedGridDisplay<'a, T, U> {
    pub grid: &'a Grid<T>,

    pub top: &'a [Option<&'a U>],
    pub left: &'a [Option<&'a U>],
    pub right: &'a [Option<&'a U>],
    pub bottom: &'a [Option<&'a U>],
}

impl<'a, T, U> SidedGridDisplay<'a, T, U> {
    pub fn new(
        grid: &'a Grid<T>,
        top: &'a [Option<&U>],
        right: &'a [Option<&U>],
        bottom: &'a [Option<&U>],
        left: &'a [Option<&U>],
    ) -> Self {
        Self {
            grid,
            top,
            bottom,
            left,
            right,
        }
    }
}

impl<'a, T, U> fmt::Display for SidedGridDisplay<'a, T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.grid.area();
        let rows = self.grid.rows();
        let cols = self.grid.cols();

        // Compute maximal width per column (grid)
        let mut max_widths = vec![0; cols];
        let mut displays = Vec::with_capacity(size);

        for (idx, cell) in self.grid.iter().enumerate() {
            let col = idx % cols;
            let display = cell.to_string();

            max_widths[col] = max_widths[col].max(display.len());

            displays.push(display);
        }

        // Compute left and right widths (sides)
        let left_width = self
            .left
            .iter()
            .map(|left| left.map(|l| l.to_string().len()).unwrap_or(0))
            .max()
            .unwrap_or(0);

        let right_width = self
            .right
            .iter()
            .map(|right| right.map(|r| r.to_string().len()).unwrap_or(0))
            .max()
            .unwrap_or(0);

        // Write the top side
        let tops: Vec<_> = self
            .top
            .iter()
            .map(|top| {
                top.map(|r| {
                    let display = r.to_string();
                    let len = display.len();

                    (display, len)
                })
            })
            .collect();

        let bottoms: Vec<_> = self
            .bottom
            .iter()
            .map(|bottom| {
                bottom.map(|r| {
                    let display = r.to_string();
                    let len = display.len();

                    (display, len)
                })
            })
            .collect();

        if self.top.iter().any(|t| t.is_some()) {
            write!(f, "{:left_width$}  ", "", left_width = left_width)?;

            for (col, top) in tops.iter().enumerate() {
                if let Some((t, tl)) = top {
                    let bl = bottoms[col].as_ref().map(|d| d.1).unwrap_or_default();

                    write!(f, " {t:<width$}", width = max_widths[col].max(*tl).max(bl))?;
                }
            }

            writeln!(f)?;
        }

        // Write the left side, grid and right side
        for row in 0..rows {
            // Left side

            if let Some(left) = self.left[row] {
                write!(f, "{:>left_width$} ", left, left_width = left_width)?;
            }

            // Grid
            write!(f, "[ ")?;

            for col in 0..cols {
                let idx = row * cols + col;
                let width = displays[idx]
                    .len()
                    .max(tops[col].as_ref().map(|d| d.1).unwrap_or_default())
                    .max(bottoms[col].as_ref().map(|d| d.1).unwrap_or_default());

                write!(f, "{:<width$} ", displays[idx], width = width)?;
            }

            write!(f, "]")?;

            // Right side
            if let Some(right) = self.right[row] {
                write!(f, " {:>right_width$}", right, right_width = right_width)?;
            }

            // Newline
            if row + 1 < rows {
                writeln!(f)?;
            }
        }

        // Write the bottom side
        if self.bottom.iter().any(|b| b.is_some()) {
            writeln!(f)?;
            write!(f, "{:left_width$}  ", "", left_width = left_width)?;

            for (col, bottom) in bottoms.iter().enumerate() {
                if let Some((b, bl)) = bottom {
                    let tl = bottoms[col].as_ref().map(|d| d.1).unwrap_or_default();

                    write!(f, " {b:<width$}", width = max_widths[col].max(*bl).max(tl))?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
