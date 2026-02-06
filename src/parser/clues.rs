use std::{borrow::Cow, collections::BTreeMap};

use crate::{Parser, Result};

#[derive(Debug, Default)]
pub(crate) struct Clues<'a> {
    across: BTreeMap<u16, Cow<'a, str>>,
    down: BTreeMap<u16, Cow<'a, str>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_clues(&mut self, flat_clues: &'a [Cow<'a, str>]) -> Result<Clues<'a>> {
        let mut clues = Clues::default();

        Ok(clues)
    }
}
