use crate::Position;

pub struct LineConstraint {
    pub line: Vec<Position>,
    pub kind: LineConstraintKind,
}

pub enum LineConstraintKind {
    Parity,
    Whisper(usize),
    Thermometer { is_slow: bool },
}
