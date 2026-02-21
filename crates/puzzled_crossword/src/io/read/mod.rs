mod checksums;
mod clues;
mod error;
mod extras;
mod grids;
mod header;
mod squares;
mod state;
mod strings;

use crate::io::{Context, Extras, Grids, Header, Strings, format, is_valid_version, read};
use std::io;

pub use error::*;
pub(crate) use state::*;

use crate::Crossword;

/// Extension trait for [`Read`](io::Read) to make reading [puzzles](Crossword) from a [binary format](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki) easier
///
/// Includes convenience methods for reading a [`u8`], [`u16`], `\0` terminated [`str`] and [`Vec<u8>`] from a generic reader
pub trait PuzRead: io::Read {
    /// Read a [`u8`]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;

        Ok(u8::from_le_bytes(bytes))
    }

    /// Read a [`u16`]
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;

        Ok(u16::from_le_bytes(buf))
    }

    /// Read a null-terminated string into a [`Vec<u8>`]
    fn read_str0(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut byte = [0];

        loop {
            self.read_exact(&mut byte)?;
            buf.push(byte[0]);

            if byte[0] == b'\0' {
                break;
            }
        }

        Ok(buf)
    }

    /// Read a [`[u8]`](core::slice) of constant size `N`
    fn read_slice<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut slice = [0; N];
        self.read_exact(&mut slice)?;

        Ok(slice)
    }

    /// Read a [`Vec`] of given size
    fn read_vec(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut vec = vec![0; len];
        self.read_exact(&mut vec)?;

        Ok(vec)
    }

    /// Skip over a given number of bytes
    fn skip(&mut self, count: usize) -> io::Result<()> {
        self.read_vec(count)?;
        Ok(())
    }
}

impl<R: io::Read> PuzRead for R {}

#[derive(Debug, Default)]
pub struct PuzReader {
    strict: bool,
}

impl PuzReader {
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    pub fn read<R: PuzRead>(&self, reader: &mut R) -> read::Result<Crossword> {
        let (puzzle, _) = self.read_with_warnings(reader)?;
        Ok(puzzle)
    }

    pub fn read_with_warnings<R: PuzRead>(
        &self,
        reader: &mut R,
    ) -> read::Result<(Crossword, Vec<Warning>)> {
        let mut state = PuzState::new(self.strict);

        // Read main components
        let header = Header::read_from(reader)?;
        if !is_valid_version(&header.version) {
            let result: read::Result<()> = Err(format::Error::InvalidVersion).context("Version");
            state.ok_or_warn(result)?;
        }

        dbg!(&header);

        let grids = Grids::read_from(reader, header.width, header.height)?;
        let strings = Strings::read_from(reader, header.clue_count)?;

        // Validate checksums
        self.validate_checksums(&header, &grids, &strings, &mut state)?;

        // Read extra sections and the actual structure of the puzzle
        let extras = Extras::read_from(reader, header.width, header.height, &mut state)?;
        let squares = self.read_squares(&grids, &extras)?;
        let clues = self.read_clues(&squares, &strings)?;

        // Build the puzzle with owned data
        let mut puzzle = Crossword::new(squares, clues)
            .with_author(build_string(&strings.author))
            .with_copyright(build_string(&strings.copyright))
            .with_notes(build_string(&strings.notes))
            .with_title(build_string(&strings.title))
            .with_version(build_string(&header.version));

        if let Some(timer) = &extras.ltim {
            *puzzle.timer_mut() = *timer;
        }

        Ok((puzzle, state.warnings))
    }

    #[cfg(feature = "miette")]
    pub fn report<T>(&self, result: Result<T>, input: &[u8]) -> miette::Result<T> {
        result.map_err(|err| {
            use miette::{NamedSource, Report};

            let source = NamedSource::new(".puz", build_string(input));
            Report::new(err).with_source_code(source)
        })
    }
}

#[derive(Debug, Default)]
pub struct TxtReader;

impl<'a> TxtReader {
    pub fn read(&self, input: &'a str) -> Result<Crossword> {
        let mut state = TxtState {
            lines: input.lines(),
            pos: 0,
            len: None,
        };

        let squares = self.parse_grid(&mut state)?;
        let mut puzzle = Crossword::from_squares(squares);

        let clues = self.parse_clues(&mut state)?;
        puzzle.insert_clues(clues);

        puzzle = self.parse_strings(puzzle, &mut state)?;

        Ok(puzzle)
    }
}

pub(crate) fn build_string(bytes: &[u8]) -> String {
    let stripped = bytes.strip_suffix(&[0]).unwrap_or(bytes);

    match std::str::from_utf8(stripped) {
        // Check if the string can be parsed as UTF-8 directly
        Ok(s) => s.to_string(),

        // Otherwise, apply the Windows-1252 character mapping
        Err(_) => stripped.iter().map(|&b| windows_1252_to_char(b)).collect(),
    }
}

pub(crate) fn windows_1252_to_char(byte: u8) -> char {
    // Windows-1252 character mapping for bytes 128-159 that differ from ISO-8859-1
    // Legacy .puz files often use Windows-1252 encoding for special characters
    match byte {
        // Standard ASCII range (0-127) maps directly
        0..=127 => byte as char,
        // Windows-1252 specific mappings for 128-159 range
        128 => '€',        // Euro sign
        129 => '\u{0081}', // Unused
        130 => '‚',        // Single low-9 quotation mark
        131 => 'ƒ',        // Latin small letter f with hook
        132 => '„',        // Double low-9 quotation mark
        133 => '…',        // Horizontal ellipsis
        134 => '†',        // Dagger
        135 => '‡',        // Double dagger
        136 => 'ˆ',        // Modifier letter circumflex accent
        137 => '‰',        // Per mille sign
        138 => 'Š',        // Latin capital letter S with caron
        139 => '‹',        // Single left-pointing angle quotation mark
        140 => 'Œ',        // Latin capital ligature OE
        141 => '\u{008D}', // Unused
        142 => 'Ž',        // Latin capital letter Z with caron
        143 => '\u{008F}', // Unused
        144 => '\u{0090}', // Unused
        145 => '\u{2018}', // Left single quotation mark
        146 => '\u{2019}', // Right single quotation mark
        147 => '\u{201C}', // Left double quotation mark
        148 => '\u{201D}', // Right double quotation mark
        149 => '•',        // Bullet
        150 => '–',        // En dash
        151 => '—',        // Em dash
        152 => '˜',        // Small tilde
        153 => '™',        // Trade mark sign
        154 => 'š',        // Latin small letter s with caron
        155 => '›',        // Single right-pointing angle quotation mark
        156 => 'œ',        // Latin small ligature oe
        157 => '\u{009D}', // Unused
        158 => 'ž',        // Latin small letter z with caron
        159 => 'Ÿ',        // Latin capital letter Y with diaeresis
        // ISO-8859-1 range (160-255) is identical to Windows-1252
        160..=255 => byte as char,
    }
}

#[cfg(test)]
mod tests {
    use crate::Crossword;
    use crate::io::{PuzReader, TxtReader, Warning, read};
    use rstest::rstest;
    use std::fs::File;
    use std::{fs, path::PathBuf};

    fn parse_puz(path: PathBuf, strict: bool) -> read::Result<(Crossword, Vec<Warning>)> {
        let mut file = File::open(path).expect("puzzle file exists");
        let parser = PuzReader::new(strict);

        parser.read_with_warnings(&mut file)
    }

    fn parse_txt(path: PathBuf) -> read::Result<Crossword> {
        let text = fs::read_to_string(&path).expect("puzzle file exists");
        let parser = TxtReader;

        parser.read(text.as_str())
    }

    #[rstest]
    fn parse_ok_puz(#[files("puzzles/ok/*.puz")] path: PathBuf) {
        let result = parse_puz(path, false);
        let (puzzle, _) = result.expect("puzzle is parsed correctly");

        assert!(puzzle.rows() > 0);
        assert!(puzzle.cols() > 0);
    }

    #[rstest]
    fn parse_ok_txt(#[files("puzzles/ok/*.txt")] path: PathBuf) {
        let result = parse_txt(path.clone());
        let puzzle = result.expect("puzzled is parsed correctly");

        assert!(puzzle.rows() > 0);
        assert!(puzzle.cols() > 0);
    }

    #[rstest]
    fn parse_err_puz(#[files("puzzles/err/*.puz")] path: PathBuf) {
        let result = parse_puz(path, true);
        let err = result.expect_err("puzzle is not parsed correctly");

        eprintln!("{err}");
    }

    #[rstest]
    fn parse_warn(#[files("puzzles/warn/*.puz")] path: PathBuf) {
        let result = parse_puz(path, false);
        let (_, warnings) = result.expect("puzzle is parsed correctly");

        assert!(!warnings.is_empty());
    }

    #[rstest]
    fn parse_same(#[files("puzzles/**/*.puz")] path: PathBuf) {
        let result = parse_puz(path.clone(), false);

        let txt_path = path.with_extension("txt");

        // Make sure .puz and .txt files parse the same if both defined
        if txt_path.exists() {
            let result2 = parse_txt(txt_path);

            match (result, result2) {
                (Ok((puzzle, _)), Ok(puzzle2)) => {
                    assert_eq!(puzzle, puzzle2);
                }
                (Ok((puzzle, _)), Err(err)) => {
                    panic!("Found left: {puzzle} and right: {err}");
                }
                (Err(err), Ok(puzzle)) => {
                    panic!("Found left: {err} and right: {puzzle}");
                }
                _ => {}
            }
        }
    }
}
