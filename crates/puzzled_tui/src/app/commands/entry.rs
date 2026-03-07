use puzzled_core::{Entry, Position};

pub struct EntryCommand<T> {
    changes: Vec<EntryChange<T>>,
}

pub struct EntryChange<T> {
    pos: Position,
    before: Entry<T>,
    after: Entry<T>,
}
