use crate::Action;

pub enum Motion {
    Direction(Action),
    TextObject(TextObject),
}

pub enum TextObject {
    Run,
    Line,
    Block,
}
