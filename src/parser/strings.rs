use crate::{Parser, Result};

#[derive(Debug)]
pub(crate) struct Strings<'a> {
    pub title: &'a [u8],
    pub author: &'a [u8],
    pub copyright: &'a [u8],
    pub notes: &'a [u8],
    pub clues: Vec<&'a [u8]>,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_strings(&mut self, clue_count: usize) -> Result<Strings<'a>> {
        // | Description | Example            |
        // |:------------|:-------------------|
        // | Title       | Theme: .PUZ format |
        // | Author      | J. Puz / W. Shortz |
        // | Copyright   | (c) 2007 J. Puz    |
        // | Clue#1      | Cued, in pool      |
        // | ...         | ...more clues...   |
        // | Clue#n      | Quiet              |
        // | Notes       | http://mywebsite   |
        let title = self.read_str("Title")?;
        let author = self.read_str("Author")?;
        let copyright = self.read_str("copyright")?;

        // Sequentially parse the clues
        let mut clues = Vec::with_capacity(clue_count);

        for num in 1..=clue_count {
            let context = format!("Clue #{num}");
            let clue = self.read_str(context)?;

            clues.push(clue);
        }

        let notes = self.read_str("Notes")?;

        Ok(Strings {
            title,
            author,
            copyright,
            notes,
            clues,
        })
    }
}
