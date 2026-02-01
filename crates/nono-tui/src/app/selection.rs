use nono::Axis;
use ratatui::layout::{Position, Rect};

use crate::MotionRange;

#[derive(Debug, Default, Clone, Copy)]
pub enum SelectionKind {
    #[default]
    Cells,

    Lines,
}

#[derive(Debug, Default, Clone)]
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

            SelectionKind::Lines => match self.axis {
                Axis::Col => {
                    let y1 = start.y.min(end.y);
                    let y2 = start.y.max(end.y);
                    MotionRange::Rows { start: y1, end: y2 }
                }
                Axis::Row => {
                    let x1 = start.x.min(end.x);
                    let x2 = start.x.max(end.x);
                    MotionRange::Cols { start: x1, end: x2 }
                }
            },
        }
    }
}
