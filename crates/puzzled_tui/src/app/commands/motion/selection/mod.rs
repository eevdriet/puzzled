mod kind;
mod multi;

pub use kind::*;
pub use multi::*;

use ratatui::layout::{Position, Rect, Size};

use crate::AsApp;

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
        P: AsApp<Position>,
    {
        if self.start.is_none() {
            self.start = Some(start.as_app());
        }

        self.end = Some(end.as_app());
    }

    pub fn update<P>(&mut self, end: P)
    where
        P: AsApp<Position>,
    {
        self.end = Some(end.as_app());
    }

    pub fn range<S>(&self, size: &S) -> Rect
    where
        S: AsApp<Size>,
    {
        let rect = Rect::from(size.as_app());

        match (self.start, self.end) {
            // Single point
            (Some(start), None) => Rect {
                x: start.x,
                y: start.y,
                width: 1,
                height: 1,
            },

            (Some(start), Some(end)) => {
                let range_w = 1 + start.x.abs_diff(end.x);
                let range_h = 1 + start.y.abs_diff(end.y);

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
                        x: start.x.min(end.x),
                        y: start.y.min(end.y),
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
