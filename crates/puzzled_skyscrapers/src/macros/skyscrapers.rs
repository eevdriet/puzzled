#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! skycrapers {
    (
        // Grid
        ($($top_clues:tt)+)
            $(
                | $left_clues:tt [$($skyscrapers:tt)+] $right_clues:tt |
            )+
        ($($bottom_clues:tt)+)

        // Metadata
        $( $meta_key:ident : $meta_value:literal )*
    ) => {{
        // Verify each row has the same width
        let mut _assert_width = [(); $crate::__count!($($top_clues)+)];
        let cols = $crate::__count!($($top_clues)+);
        let rows = 0usize;

        $(
            let _assert_width_row = [(); $crate::__count!($($skyscrapers)+)];
            _assert_width = _assert_width_row;
            let rows = rows + 1usize;
        )+

        let _assert_width_bottom = [(); $crate::__count!($($bottom_clues)+)];
        _assert_width = _assert_width_bottom;

        // Construct the skyscrapers
        let skyscrapers = $crate::grid![
            $([$( $crate::skyscraper!($skyscrapers) ),+]),+
        ];

        let skyscrapers = skyscrapers.map(|s| $crate::Cell::new(s));

        // Construct the clues
        let mut clues = std::collections::BTreeMap::default();

        let top = [$($crate::__clue_value!($top_clues)),+];
        let left = [$($crate::__clue_value!($left_clues)),+];
        let right = [$($crate::__clue_value!($right_clues)),+];
        let bottom = [$($crate::__clue_value!($bottom_clues)),+];

        $crate::__insert_clues!(clues, top, $crate::Direction::Down, Col);
        $crate::__insert_clues!(clues, left, $crate::Direction::Right, Row);
        $crate::__insert_clues!(clues, right, $crate::Direction::Left, Row);
        $crate::__insert_clues!(clues, bottom, $crate::Direction::Up, Col);

        // Add metadata
        let meta = $crate::metadata!($( $meta_key : $meta_value),*);

        // Create puzzle
        $crate::Skyscrapers::new(skyscrapers, $crate::Clues::new(clues, rows, cols), meta)
    }};
}

#[cfg(test)]
mod tests {
    use puzzled_io::TxtPuzzle;

    use crate::Skyscrapers;

    #[test]
    fn write() {
        let puzzle = skycrapers!(
            (     1 2 3       )
            | 4 [ - - - ] .   |
            | 5 [ - - - ] .   |
            | 6 [ - - - ] .   |
            (     7 8 9       )
            version: "1.0"
        );

        let _ = puzzle.save_text("test");
        assert!(false);
    }

    #[test]
    fn read() {
        let puzzle = Skyscrapers::load_text("read").expect("Should read correctly");
    }
}
