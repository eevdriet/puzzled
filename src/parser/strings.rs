use std::borrow::Cow;

use crate::{Parser, Region, Result};

#[derive(Debug)]
pub(crate) struct Strings<'a> {
    // Strings
    pub title: Cow<'a, str>,
    pub author: Cow<'a, str>,
    pub copyright: Cow<'a, str>,
    pub notes: Cow<'a, str>,
    pub clues: Vec<Cow<'a, str>>,

    // Regions
    pub title_region: Region<'a>,
    pub author_region: Region<'a>,
    pub copyright_region: Region<'a>,
    pub notes_region: Region<'a>,
    pub clue_regions: Vec<Region<'a>>,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_strings(&mut self, clue_count: u16) -> Result<Strings<'a>> {
        // | Description | Example            |
        // |:------------|:-------------------|
        // | Title       | Theme: .PUZ format |
        // | Author      | J. Puz / W. Shortz |
        // | Copyright   | (c) 2007 J. Puz    |
        // | Clue#1      | Cued, in pool      |
        // | ...         | ...more clues...   |
        // | Clue#n      | Quiet              |
        // | Notes       | http://mywebsite   |
        let (title, title_region) = self.read_region(|p| p.read_str("Title"))?;
        let (author, author_region) = self.read_region(|p| p.read_str("Author"))?;
        let (copyright, copyright_region) = self.read_region(|p| p.read_str("copyright"))?;

        // Sequentially parse the clues
        let mut clues = Vec::new();
        let mut clue_regions = Vec::new();

        for num in 1..=clue_count {
            let context = format!("Clue #{num}");
            let (clue, clue_region) = self.read_region(|p| p.read_str(context))?;

            clues.push(clue);
            clue_regions.push(clue_region);
        }

        let (notes, notes_region) = self.read_region(|p| p.read_str("Notes"))?;

        Ok(Strings {
            // Strings
            title,
            author,
            copyright,
            notes,
            clues,

            // Regions
            title_region,
            author_region,
            copyright_region,
            notes_region,
            clue_regions,
        })
    }
}
