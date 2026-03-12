use derive_more::Display;
use puzzled_core::Position;
use ratatui::layout::{Rect, Size};
use serde::Deserialize;

use crate::{AsApp, AsCore};

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Display)]
pub enum SelectionKind {
    #[default]
    Cells,

    Rows,
    Cols,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Selection {
    start: Option<Position>,
    end: Option<Position>,
    kind: SelectionKind,
}

impl Selection {
    pub fn start(&self) -> Option<Position> {
        self.start
    }

    pub fn end(&self) -> Option<Position> {
        self.end
    }

    pub fn set_kind(&mut self, kind: SelectionKind) {
        self.kind = kind;
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.end = None;
    }

    pub fn set<P>(&mut self, start: P, end: P)
    where
        P: AsCore<Position>,
    {
        if self.start.is_none() {
            self.start = Some(start.as_core());
        }

        self.end = Some(end.as_core());
    }

    pub fn update<P>(&mut self, end: P)
    where
        P: AsCore<Position>,
    {
        self.end = Some(end.as_core());
    }

    pub fn range<S>(&self, size: S) -> Rect
    where
        S: AsApp<Size>,
    {
        let rect = Rect::from(size.as_app());

        match (self.start, self.end) {
            // Single point
            (Some(start), None) => Rect {
                x: start.col as u16,
                y: start.row as u16,
                width: 1,
                height: 1,
            },

            (Some(start), Some(end)) => {
                let range_w = 1 + start.col.abs_diff(end.col) as u16;
                let range_h = 1 + start.row.abs_diff(end.row) as u16;

                match self.kind {
                    SelectionKind::Rows => Rect {
                        height: range_h,
                        ..rect
                    },
                    SelectionKind::Cols => Rect {
                        width: range_w,
                        ..rect
                    },
                    SelectionKind::Cells => Rect {
                        x: start.col.min(end.col) as u16,
                        y: start.row.min(end.row) as u16,
                        width: range_w,
                        height: range_h,
                    },
                }
            }

            // Other (no points)
            _ => Rect::default(),
        }
    }
}
