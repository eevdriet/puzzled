use ratatui::layout::{Position, Rect};

#[derive(Debug, Clone, Copy)]
pub enum ActionKind {
    Operator,

    Motion,

    Mode,

    Command,
}

#[derive(Debug, Clone, Default)]
pub enum MotionRange {
    #[default]
    Empty,

    Single(Position),
    Block(Rect),
    Rows {
        start: u16,
        end: u16,
    },
    Cols {
        start: u16,
        end: u16,
    },
}

impl MotionRange {
    pub fn contains(&self, pos: Position) -> bool {
        match self {
            MotionRange::Empty => false,
            MotionRange::Single(single) => *single == pos,
            MotionRange::Block(block) => block.contains(pos),
            MotionRange::Rows { start, end } => (*start..=*end).contains(&pos.y),
            MotionRange::Cols { start, end } => (*start..=*end).contains(&pos.x),
        }
    }

    pub fn positions(&self, bounds: &Rect) -> Vec<Position> {
        let from_rect = |rect: Rect| -> Vec<Position> {
            let mut positions = Vec::new();

            for x in rect.x..rect.x + rect.width {
                for y in rect.y..rect.y + rect.height {
                    let pos = Position::new(x, y);
                    positions.push(pos);
                }
            }

            positions
        };

        match self {
            MotionRange::Single(pos) => {
                if bounds.contains(*pos) {
                    vec![*pos]
                } else {
                    vec![]
                }
            }
            MotionRange::Rows { start, end } => {
                let y = bounds.y + *start;
                let height = end - start + 1;

                let rect = Rect::new(bounds.x, y, bounds.width, height);
                let rect = rect.intersection(*bounds);

                from_rect(rect)
            }
            MotionRange::Cols { start, end } => {
                let x = bounds.x + *start;
                let width = end - start + 1;

                let rect = Rect::new(x, bounds.y, width, bounds.height);
                let rect = rect.intersection(*bounds);

                from_rect(rect)
            }
            MotionRange::Block(rect) => {
                let rect = rect.intersection(*bounds);

                from_rect(rect)
            }
            _ => vec![],
        }
    }
}
