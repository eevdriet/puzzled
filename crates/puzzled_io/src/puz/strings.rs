use std::fmt::{self, Display};

use puzzled_core::Metadata;

use crate::puz::{Context, PuzRead, PuzWrite, build_string, read, write};

#[derive(Debug)]
pub struct ByteStr(Vec<u8>);

impl ByteStr {
    pub fn new(bytes: &[u8]) -> Self {
        let mut bytes = bytes.to_vec();
        if bytes.last().is_none_or(|byte| *byte != b'\0') {
            bytes.push(b'\0');
        }

        Self(bytes)
    }

    pub fn str_len(&self) -> usize {
        self.0
            .len()
            .checked_sub(1)
            .expect("\\0 trailing byte set at construction")
    }

    pub fn is_empty(&self) -> bool {
        self.str_len() == 0
    }

    pub fn bytes(&self, include_0: bool) -> &[u8] {
        if include_0 {
            &self.0
        } else {
            &self.0[..self.0.len() - 1]
        }
    }
}

impl fmt::Display for ByteStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", build_string(self.bytes(false)))
    }
}

impl Default for ByteStr {
    fn default() -> Self {
        Self(vec![b'\0'])
    }
}

/// [Strings](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#strings-section) section
///
/// This section ncludes the "metadata" for the [puzzle](crate::Puz).
/// It also includes the text for every [clue](puzzled::crossword::Clue) that should be placed in the puzzle [squares](puzzled::crossword::Squares).
/// The pseudo-code of how to do so is given [here](https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5#clue-assignment) and the crate implementation is found in [`Crossword::place_clues`](crate::Crossword::place_clues).
///
/// Below is an example of the order in which the strings should be read:
/// | Component | Length | Type | Example |
/// |:------------|:-------------------|:---|:---|
/// | Title       | ? | str | Theme: .PUZ format |
/// | Author      | ? | str | J. Puz / W. Shortz |
/// | Copyright   | ? | str | (c) 2007 J. Puz    |
/// | Clue `#1`   | ? | str | Cued, in pool      |
/// | ...         | ... | ... | ... |
/// | Clue `#n`   | ? | str | Quiet              |
/// | Notes       | ? | str | http://mywebsite   |
#[derive(Debug, Default)]
pub struct Strings {
    /// Title of the puzzle
    pub title: ByteStr,

    /// Author of the puzzle
    pub author: ByteStr,

    /// Copyright of the puzzle
    pub copyright: ByteStr,

    /// Notes on the puzzle
    pub notes: ByteStr,

    /// Clues to be placed in the puzzle
    pub clues: Vec<ByteStr>,
}

/// # Read
impl Strings {
    pub fn from_metadata(meta: &Metadata) -> Self {
        let to_byte_str =
            |prop: Option<&str>| prop.map(|p| ByteStr::new(p.as_bytes())).unwrap_or_default();

        Strings {
            author: to_byte_str(meta.author()),
            copyright: to_byte_str(meta.copyright()),
            notes: to_byte_str(meta.notes()),
            title: to_byte_str(meta.title()),

            clues: Vec::new(),
        }
    }

    pub(crate) fn read_from<R: PuzRead>(reader: &mut R, clue_count: u16) -> read::Result<Self> {
        let title = reader.read_byte_str().context("Title")?;
        let author = reader.read_byte_str().context("Author")?;
        let copyright = reader.read_byte_str().context("Copyright")?;

        // Sequentially parse the clues
        let mut clues = Vec::with_capacity(clue_count as usize);

        for num in 1..=clue_count {
            let context = format!("Clue #{num}");
            let clue = reader.read_byte_str().context(context)?;

            clues.push(clue);
        }

        let notes = reader.read_byte_str().context("Notes")?;

        Ok(Strings {
            title,
            author,
            copyright,
            notes,
            clues,
        })
    }
}

/// # Write
impl Strings {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        writer.write_byte_str(&self.title).context("Title")?;
        writer.write_byte_str(&self.author).context("Author")?;
        writer
            .write_byte_str(&self.copyright)
            .context("Copyright")?;

        for (idx, clue) in self.clues.iter().enumerate() {
            let num = idx + 1;
            let context = format!("Clue #{num}");
            writer.write_byte_str(clue).context(context)?;
        }

        writer.write_byte_str(&self.notes).context("Notes")?;

        Ok(())
    }
}
