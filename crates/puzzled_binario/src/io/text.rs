use puzzled_io::{TxtPuzzle, text::read};

use crate::{Binario, BinarioState};

impl TxtPuzzle<BinarioState> for Binario {
    fn read_text(_reader: &mut read::TxtState) -> read::Result<Self> {
        Err(read::Error::Io(std::io::Error::other("err")))
    }
}
