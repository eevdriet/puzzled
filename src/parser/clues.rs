use std::collections::BTreeMap;

use crate::{Parser, Result};

#[derive(Debug, Default)]
pub(crate) struct Clues<'a> {
    across: BTreeMap<u16, &'a [u8]>,
    down: BTreeMap<u16, &'a [u8]>,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_clues(&mut self, flat_clues: &'a [&'a [u8]]) -> Result<Clues<'a>> {
        let mut clues = Clues::default();

        Ok(clues)
    }
}
