use crate::{format, text::read};

#[derive(Debug)]
pub struct TxtState<'a> {
    pub(crate) strict: bool,
    pub(crate) warnings: Vec<read::Error>,

    pub(crate) remainder: &'a str,
    pub(crate) peeked: Option<&'a str>,

    pub(crate) pos: usize,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl<'a> TxtState<'a> {
    pub fn new(input: &'a str, strict: bool) -> Self {
        Self {
            strict,
            warnings: Vec::new(),
            remainder: input,
            peeked: None,

            pos: 0,
            line: 0,
            col: 0,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.remainder.is_empty()
    }

    pub(crate) fn advance(&mut self, len: usize) {
        self.remainder = &self.remainder[len..];
        self.pos += len;
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.remainder.chars().next()
    }

    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        let len = ch.len_utf8();

        self.advance(len);

        match ch {
            '\n' => {
                self.line += 1;
                self.col = 1;
            }
            _ => {
                self.col += 1;
            }
        }

        Some(ch)
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }

            self.next_char();
        }
    }

    pub fn peek_line(&mut self) -> Option<&'a str> {
        if self.peeked.is_some() {
            return self.peeked;
        }

        let newline = self.remainder.find('\n')?;
        let line = &self.remainder[..newline].trim();

        if line.is_empty() {
            self.advance(newline);

            return self.peek_line();
        }

        self.peeked = Some(line);
        self.peeked
    }

    pub fn next_line(&mut self) -> Option<&'a str> {
        let line = match self.peeked.take() {
            Some(line) => {
                self.line += 1;
                self.col = 1;

                line
            }
            None => match self.remainder.find('\n') {
                Some(newline) => {
                    let line = &self.remainder[..newline];
                    self.line += 1;
                    self.col = 1;

                    line
                }
                None => {
                    let line = self.remainder;
                    self.remainder = "";
                    self.col += line.len();

                    line
                }
            },
        };

        self.advance(line.len());

        let line = line.trim();
        if line.is_empty() {
            return self.next_line();
        }

        Some(line)
    }

    pub fn next_prefixed(&mut self, prefix: &str) -> Option<&'a str> {
        self.peek_line()?.strip_prefix(prefix)?;

        self.next_line()?.strip_prefix(prefix)
    }

    pub fn next_delimited(&mut self, prefix: &str, suffix: &str) -> Option<&'a str> {
        self.peek_line()?
            .strip_prefix(prefix)?
            .strip_suffix(suffix)?;

        self.next_line()?.strip_prefix(prefix)?.strip_suffix(suffix)
    }

    pub fn read_string(&self, text: &str) -> read::Result<String> {
        let text = text.trim();

        if !text.starts_with('"') || !text.ends_with('"') {
            let err = format::StringError::InvalidLiteral {
                found: text.to_string(),
            };
            return Err(format::Error::String(err).into());
        }

        Ok(text[1..text.len() - 1].to_string())
    }

    pub fn ok_or_warn<T>(&mut self, result: read::Result<T>) -> read::Result<Option<T>> {
        match result {
            // Pass through ok/err with strict mode normally
            Ok(val) => Ok(Some(val)),
            Err(err) if self.strict => Err(err),

            // Warn against errors in non-strict mode
            Err(warning) => {
                self.warnings.push(warning);

                Ok(None)
            }
        }
    }
}
