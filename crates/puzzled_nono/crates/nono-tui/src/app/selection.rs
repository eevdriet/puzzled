use nono::Axis;
use ratatui::layout::{Position, Rect};

use crate::MotionRange;

#[derive(Debug, Default, Clone, Copy)]
pub enum SelectionKind {
    #[default]
    Cells,

    Rows,
    Cols,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Selection {
    pub axis: Axis,
    pub kind: SelectionKind,

    start: Option<Position>,
    end: Option<Position>,
}

impl Selection {
    pub fn empty(axis: Axis) -> Self {
        Self {
            axis,
            kind: SelectionKind::default(),
            start: None,
            end: None,
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.end = None;
    }

    pub fn start(&mut self, start: Position, kind: SelectionKind) {
        self.kind = kind;
        self.start = Some(start);
    }

    pub fn update(&mut self, end: Position) {
        self.end = Some(end);
    }

    pub fn contains(&self, pos: Position) -> bool {
        let range = self.range();
        range.contains(pos)
    }

    pub fn range(&self) -> MotionRange {
        let (start, end) = match (self.start, self.end) {
            (Some(s), Some(e)) => (s, e),
            (Some(s), None) => (s, s),
            _ => return MotionRange::Empty,
        };

        match self.kind {
            SelectionKind::Cells => {
                let x1 = start.x.min(end.x);
                let y1 = start.y.min(end.y);
                let x2 = start.x.max(end.x);
                let y2 = start.y.max(end.y);

                let block = Rect::new(x1, y1, x2 - x1 + 1, y2 - y1 + 1);
                MotionRange::Block(block)
            }

            SelectionKind::Rows => {
                let y1 = start.y.min(end.y);
                let y2 = start.y.max(end.y);
                MotionRange::Rows { start: y1, end: y2 }
            }

            SelectionKind::Cols => {
                let x1 = start.x.min(end.x);
                let x2 = start.x.max(end.x);
                MotionRange::Cols { start: x1, end: x2 }
            }
        }
    }
}
