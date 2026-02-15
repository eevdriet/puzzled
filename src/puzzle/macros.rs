#[doc(hidden)]
#[macro_export]
macro_rules! __count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::__count!($($xs)*));
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dir {
    (A) => {
        $crate::Direction::Across
    };
    (D) => {
        $crate::Direction::Down
    };
}

#[macro_export]
macro_rules! clue {
    ($dir:ident : $clue:literal) => {
        $crate::ClueSpec::new($crate::__dir!($dir), $clue)
    };
}

#[macro_export]
macro_rules! clue_at {
    ($num:literal $dir:ident : $clue:literal @ ($row:literal, $col:literal) + $len:literal) => {
        $crate::Clue::new(
            $num,
            $crate::__dir!($dir),
            $clue,
            $crate::Position::new($row, $col),
            $len,
        )
    };
}

#[macro_export]
macro_rules! square {
    () => {
        $crate::Square::Black
    };
    (.) => {
        $crate::Square::Black
    };

    ($ch:tt) => {{
        // Convert letters A..Z / a..z into char
        let letters = stringify!($ch);
        let solution = match letters.len() {
            1 => $crate::Solution::Letter(letters.chars().next().unwrap()),
            _ => $crate::Solution::Rebus(letters.to_string()),
        };

        $crate::Square::White($crate::Cell::new(solution))
    }};
}

#[macro_export]
macro_rules! puzzle {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Clue definitions
        $(--- $( $dir:ident : $clue:literal ),* $( , )?)?
    ) => {{
        // Manually count the number of columns in the first row
        let mut _assert_width0 = [(); $crate::__count!($($x0)*)];
        let cols = $crate::__count!($($x0)*);
        let rows = 1usize;

        // Count the number of columns in subsequent rows
        $(
            let _assert_width = [(); $crate::__count!($($x)*)];
            _assert_width0 = _assert_width;
            let rows = rows + 1usize;
        )*

        let mut vec = Vec::with_capacity(rows.checked_mul(cols).unwrap());

        // $( vec.push($crate::square!($x0)); )*
        $( vec.push($crate::square!($x0)); )*
        $( $( vec.push($crate::square!($x)); )* )*

        let squares = $crate::Grid::new(vec, cols as u8).unwrap();

        #[allow(unused_mut)]
        let mut clues = Vec::new();

        $(
            $(
                let clue = $crate::clue!($dir : $clue);
                clues.push(clue);
            )*
        )?

        let mut puzzle = $crate::Puzzle::from_squares(squares);
        puzzle.insert_clues(clues);

        puzzle
    }};

}
