#[doc(hidden)]
#[macro_export]
macro_rules! __dir {
    (A) => {
        $crate::Direction::Across
    };
    (D) => {
        $crate::Direction::Down
    };

    ($dir:ident) => {
        compile_error!("Invalid direction: only A (across) and D (down) allowed")
    };
}

/// Inline constructor for a [clue specification](crate::ClueSpec)
#[macro_export]
macro_rules! clue_spec {
    ($dir:ident : $clue:literal) => {
        $crate::ClueSpec::new($crate::__dir!($dir), $clue)
    };
}

/// Inline constructor for a [clue](crate::Clue)
#[macro_export]
macro_rules! clue {
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

/// Inline constructor for a [cell](crate::Cell)
#[macro_export]
macro_rules! cell {
    ($lit:literal) => {{
        trait __IntoSolution {
            fn into_solution(self) -> $crate::Solution;
        }

        impl __IntoSolution for char {
            fn into_solution(self) -> $crate::Solution {
                $crate::Solution::Letter(self)
            }
        }

        impl __IntoSolution for &str {
            fn into_solution(self) -> $crate::Solution {
                $crate::Solution::Rebus(self.to_string())
            }
        }

        let solution = __IntoSolution::into_solution($lit);
        $crate::Cell::new(solution)
    }};
}

/// Inline constructor for a [square](crate::Square)
#[macro_export]
macro_rules! square {
    () => {
        $crate::Square::Black
    };
    (.) => {
        $crate::Square::Black
    };

    ($lit:literal) => {{
        let cell = $crate::cell!($lit);
        $crate::Square::White(cell)
    }};

    ($ident:ident) => {{
        let s = stringify!($ident);

        let solution = match s.len() {
            1 => $crate::Solution::Letter(s.chars().next().unwrap()),
            _ => $crate::Solution::Rebus(s.to_string()),
        };
        let cell = $crate::Cell::new(solution);

        $crate::Square::White(cell)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __metadata {
    ($puzzle:ident, author : $val:literal) => {
        $puzzle = $puzzle.with_author($val);
    };

    ($puzzle:ident, copyright : $val:literal) => {
        $puzzle = $puzzle.with_copyright($val);
    };

    ($puzzle:ident, notes : $val:literal) => {
        $puzzle = $puzzle.with_notes($val);
    };

    ($puzzle:ident, title : $val:literal) => {
        $puzzle = $puzzle.with_title($val);
    };

    ($puzzle:ident, version : $val:literal) => {
        $puzzle = $puzzle.with_version($val);
    };

    ($puzzle:ident, $key:ident : $val:literal) => {
        compile_error!(concat!("Invalid puzzle property: ", stringify!($key)));
    };
}

/// Inline constructor for a [puzzle](crate::Puzzle)
#[macro_export]
macro_rules! puzzle {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Clue definitions
        $(- $dir:ident : $clue:literal )*

        // Metadata
        $( $meta_key:ident : $meta_val:literal )*
    ) => {{
        // Add squares
        let squares = $crate::grid![
            [$( $crate::square!($x0) ),+]
            $(, [$( $crate::square!($x) ),+] )*
        ];

        // Add clues
        #[allow(unused_mut)]
        let mut clues = Vec::new();

        $(
            let clue = $crate::clue_spec!($dir : $clue);
            clues.push(clue);
        )*

        let mut puzzle = $crate::Puzzle::from_squares(squares);
        puzzle.insert_clues(clues);

        // Add metadata
        $(
            $crate::__metadata!(puzzle, $meta_key : $meta_val);
        )*

        puzzle
    }};

}
