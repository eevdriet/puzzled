use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    #[default]
    Top = 0,

    Right = 1,
    Bottom = 2,
    Left = 3,
}

pub type Sides<T> = [Option<T>; 4];

impl Side {
    pub const ALL: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];

    pub fn is_horizontal(&self) -> bool {
        matches!(self, Side::Left | Side::Right)
    }
    pub fn is_vertical(&self) -> bool {
        matches!(self, Side::Top | Side::Bottom)
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Side::Top => 'T',
                Side::Bottom => 'B',
                Side::Left => 'L',
                Side::Right => 'R',
            }
        )
    }
}
