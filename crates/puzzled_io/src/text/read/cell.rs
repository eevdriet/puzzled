use std::str::FromStr;

use puzzled_core::{Cell, CellStyle, Entry, NON_PLAYABLE_CHAR, Square};

use crate::text::{TxtState, read::CellText};

type CellEntry<T> = (Square<Cell<T>>, Square<Entry<T>>);

impl<'a> TxtState<'a> {
    pub fn parse_cell_entry<T>(&mut self) -> Option<(Cell<T>, Entry<T>)>
    where
        T: FromStr,
    {
        // Try to read the cell
        let value = self.parse_until::<T, _>(|ch| !ch.is_cell_contents(), false)?;
        let style = self
            .parse_until::<CellStyle, _>(|ch| !ch.is_cell_style(), false)
            .unwrap_or_default();
        let cell = Cell::new_with_style(Some(value), style);

        // Try to read the entry
        let value = self.parse_until::<T, _>(|ch| !ch.is_cell_contents(), true);
        let entry = Entry::new_with_style(value, style);

        Some((cell, entry))
    }

    pub fn parse_square_entry<T>(&mut self) -> Option<CellEntry<T>>
    where
        T: FromStr,
    {
        // Try to read the cell
        self.skip_whitespace();
        if let Some(start) = self.peek_char()
            && start == NON_PLAYABLE_CHAR
        {
            return Some((Square::new_empty(), Square::new_empty()));
        }

        let value = self.parse_until::<T, _>(|ch| !ch.is_cell_contents(), false)?;
        let style = self
            .parse_until::<CellStyle, _>(|ch| !ch.is_cell_style(), false)
            .unwrap_or_default();
        let cell = Cell::new_with_style(Some(value), style);

        // Try to read the entry
        let value = self.parse_until::<T, _>(|ch| !ch.is_cell_contents(), true);
        let entry = Entry::new_with_style(value, style);

        Some((Square::new(cell), Square::new(entry)))
    }

    fn parse_until<T, F>(&mut self, mut stop_fn: F, delimited: bool) -> Option<T>
    where
        T: FromStr,
        F: FnMut(char) -> bool,
    {
        self.skip_whitespace();
        let ch = self.peek_char()?;

        if delimited {
            println!("Delimter 1: {ch}");
            if ch != '(' {
                return None;
            }

            self.next_char();
        }
        self.skip_whitespace();

        let mut end = 0;

        for (idx, ch) in self.remainder.char_indices() {
            if stop_fn(ch) {
                break;
            }

            end = idx + ch.len_utf8();
        }

        if end == 0 {
            println!("End == 0");
            return None;
        }

        self.skip_whitespace();

        let token = &self.remainder[..end];

        if delimited {
            if self.remainder.get(end..=end).is_none_or(|ch| ch != ")") {
                println!("Delimter 2: {ch}");
                return None;
            }

            self.advance(end);
            self.next_char();
        } else {
            self.advance(end);
        }

        token.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use puzzled_core::CellStyle;
    use rstest::rstest;

    use crate::text::TxtState;

    const _E: CellStyle = CellStyle::empty();
    const _I: CellStyle = CellStyle::INCORRECT;
    const _P: CellStyle = CellStyle::PREVIOUSLY_INCORRECT;
    const _R: CellStyle = CellStyle::REVEALED;
    const _C: CellStyle = CellStyle::CIRCLED;

    #[rstest]
    #[case("10", Some(10), None, _E)]
    #[case("10*", Some(10), None, _R)]
    #[case("10*@", Some(10), None, _R | _C)]
    #[case("10*@ 10", Some(10), None, _R | _C)]
    #[case("10*@ (10)", Some(10), Some(10), _R | _C)]
    #[case("10*@ (22)", Some(10), Some(22), _R | _C)]
    fn cell_entry(
        #[case] input: &str,
        #[case] cell_val: Option<usize>,
        #[case] entry_val: Option<usize>,
        #[case] style: CellStyle,
    ) {
        let mut state = TxtState::new(input, false);
        let (cell, entry) = state.parse_cell_entry().expect("Can read cell entry");

        assert_eq!(cell_val, cell.solution);
        assert_eq!(style, cell.style);
        assert_eq!(entry_val.as_ref(), entry.entry());
    }
}
