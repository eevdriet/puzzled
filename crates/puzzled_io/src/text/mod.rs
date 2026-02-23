pub mod read;
pub mod write;

pub use read::{TxtReader, TxtState};

pub trait TxtPuzzle: Sized {
    fn from_text(reader: &mut read::TxtState) -> read::Result<Self>;
}
