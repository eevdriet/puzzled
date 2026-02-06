use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Across,
    Down,
}

#[derive(Debug, Clone)]
pub struct Clue {
    pub num: u8,
    pub direction: Direction,
    pub text: String,
}

impl Ord for Clue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num
            .cmp(&other.num)
            .then(self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for Clue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Clue {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.direction == other.direction
    }
}

impl Eq for Clue {}
