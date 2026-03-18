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

type Side<T> = Option<Vec<T>>;

pub struct SidedGrid<T, U> {
    pub grid: Grid<T>,

    pub top: Side<U>,
    pub right: Side<U>,
    pub bottom: Side<U>,
    pub left: Side<U>,
}

impl<T, U> SidedGrid<T, U> {
    pub fn new(
        grid: Grid<T>,
        top: Side<U>,
        right: Side<U>,
        bottom: Side<U>,
        left: Side<U>,
    ) -> Result<Self, SidedGridError> {
        for (side_str, side) in [
            ("top", top.as_ref()),
            ("right", right.as_ref()),
            ("bottom", bottom.as_ref()),
            ("left", left.as_ref()),
        ] {
            if side.is_some_and(|s| s.len() != grid.cols()) {
                return Err(SidedGridError::InvalidColCount {
                    side: side_str.to_string(),
                    found: side.expect("Checked for is_some").len(),
                    expected: grid.cols(),
                });
            }
        }

        Ok(SidedGrid {
            grid,
            top,
            bottom,
            left,
            right,
        })
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
