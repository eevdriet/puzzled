use crate::{
    Puzzle,
    io::{
        PuzWriter,
        write::{Grids, Header, PuzWrite},
    },
};

impl PuzWriter {
    pub(crate) fn write_checksums(
        &self,
        header: &mut Header,
        grids: &Grids,
        strings: &Vec<u8>,
    ) -> std::io::Result<()> {
        Ok(())
    }
}
