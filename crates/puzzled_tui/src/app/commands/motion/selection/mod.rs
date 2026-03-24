mod kind;
mod multi;

pub use kind::*;
pub use multi::*;

use ratatui::layout::{Position, Rect};

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

    pub fn range(&self, rect: Rect) -> Rect {
        let (start, end) = match (self.start, self.end) {
            (Some(s), Some(e)) => (s, e),
            (Some(s), None) => (s, s), // treat as single-cell selection
            _ => return Rect::default(),
        };

        // Normalize (top-left → bottom-right)
        let x1 = start.x.min(end.x);
        let y1 = start.y.min(end.y);
        let x2 = start.x.max(end.x);
        let y2 = start.y.max(end.y);

        let width = x2 - x1 + 1;
        let height = y2 - y1 + 1;

        let area = match self.kind {
            SelectionKind::Cells => Rect {
                x: x1,
                y: y1,
                width,
                height,
            },

            SelectionKind::Rows => Rect {
                x: 0,
                y: y1,
                height,
                ..rect
            },

            SelectionKind::Cols => Rect {
                x: x1,
                y: 0,
                width,
                ..rect
            },
        };

        area.intersection(rect)
    }
}
