/// Macro for constructing a [crossword](crate::Crossword) inline
///
/// A crossword is constructed from the following three sections
/// 1. [Squares](crate::Squares) grid
///
///    It contains the [solution](crate::Solution) to each [square](crate::Square).
///    The syntax for constructing the squares is analoguous to that of using [`grid!`](crate::grid) where each entry is a [`square!`](crate::square)
/// 2. **(Optionally)** [Clues](crate::Clues) list
///
///    Each [clue](crate::Clue) is placed in the crossword using [`Crossword::place_clues`](crate::Crossword::place_clues), which does not require a strict order among the clues.
///    Therefore, you can place all `A` clues first or mix them with the `D` clues -- as long as all `A` clues and `D` clues are ordered amongst themselves.
///
///    The syntax for constructing the clues is analoguous to listing each clue with [`clue_spec!`](crate::clue_spec).
/// 3. **(Optionally)** Metadata
///
///    To further define the crossword, you can specificy metadata such as its [title](crate::Crossword::title) and [author](crate::Crossword::author).
///    Each property is set as `<key>: <val>`, where `<val>` is expected to be a string literal
///
/// ```
/// use puzzled::crossword::{crossword, clue_spec, Direction::*, Position, square};
///
/// let puzzle = crossword! (
///     [C A N .]
///     [A G E .]
///     [R O Wordle .]
///     - A: "To be able to"
///     - D: "An automobile"
///     - A: "The length of life"
///     - D: "Past, gone, before now"
///     - A: "Some stuff arranged in a line"
///     - D: "Not existing before"
///     title: "My crossword"
/// );
/// // Squares
/// assert_eq!(puzzle[Position::new(0, 3)], square!(.));
/// assert_eq!(puzzle[Position::new(1, 1)], square!('G'));
/// assert_eq!(puzzle[Position::new(2, 2)], square!("Wordle"));
///
/// // Clues
/// assert_eq!(puzzle[(1, Across)].spec(), clue_spec!(A: "To be able to"));
/// assert_eq!(puzzle[(1, Down)].spec(), clue_spec!(D: "An automobile"));
///
/// // Metadata
/// assert_eq!(puzzle.rows(), 3);
/// assert_eq!(puzzle.cols(), 4);
/// assert_eq!(puzzle.title(), Some("My crossword"));
/// assert_eq!(puzzle.author(), None);
/// ```
#[macro_export]
macro_rules! crossword {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Clue definitions
        $(- $dir:ident : $clue:literal )*

        // Metadata
        $( $meta_key:ident : $meta_val:literal )*
    ) => {{
        // Add squares
        let grid = $crate::grid![
            [$( $crate::square!($x0) ),+]
            $(, [$( $crate::square!($x) ),+] )*
        ];
        let squares = $crate::Squares::new(grid);

        // Add clues
        #[allow(unused_mut)]
        let mut clues = Vec::new();

        $(
            let clue = $crate::clue_spec!($dir : $clue);
            clues.push(clue);
        )*

        let mut puzzle = $crate::Crossword::from_squares(squares);
        puzzle.insert_clues(clues);

        // Add metadata
        $(
            $crate::__metadata!(puzzle, $meta_key : $meta_val);
        )*

        puzzle
    }};

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid:tt)*, "crossword!")
    }};
}

/// Macro for constructing a [square](crate::Square) inline
///
/// Squares can be constructed in 3 different ways:
/// 1. Empty or `.` constructs a [black square](crate::Square::Black)
/// 2. A [`char`] constructs a [white square](crate::Square::White) with a [letter](crate::Solution::Letter) [cell](crate::Cell)
/// 3. A [`str`] constructs a white square with a [rebus](crate::Solution::Rebus) cell
///
/// Note that for the latter two, the syntax is analoguous to using [`cell!`](crate::cell).
/// ```
/// use puzzled::crossword::{square, Cell, cell};
///
/// assert_eq!(square!(), None::<Cell>);
/// assert_eq!(square!(.), None::<Cell>);
/// assert_eq!(square!('L'), Some(cell!('L')));
/// assert_eq!(square!("Rebus"), Some(cell!("Rebus")));
///
/// // Invalid syntax
/// // square!(9)
/// // square!(L)
/// // square!(ABC)
/// ```
#[macro_export]
macro_rules! square {
    () => {
        None
    };
    (.) => {
        None
    };

    ($lit:literal) => {{
        Some($crate::cell!($lit))
    }};

    ($ident:ident) => {{
        let s = stringify!($ident);

        let solution = match s.len() {
            1 => $crate::Solution::Letter(s.chars().next().unwrap()),
            _ => $crate::Solution::Rebus(s.to_string()),
        };

        let cell = $crate::Cell::new(solution);
        Some(cell)
    }};

    ($($invalid:tt)*) => {{
        $crate::__error($($tt:tt)*, "square!")
    }};
}

/// Macro for constructing a [clue specification](crate::ClueSpec) inline
///
/// A specification is constructed as
/// ```bnf
/// <spec> ::= <dir> : <text>
/// <dir> ::= A | D
/// ```
/// where `A` and `D` respectively denote the [`Across`](crate::Direction::Across) and [`Down`](crate::Direction::Down) `<dir>` the clue can be placed in.
/// The `<text>` can be any literal that is [`Into<String>`] -- so e.g. [`char`] is allowed for single letter clues.
///
/// To construct a placed [clue](crate::Clue) you can use the [`clue!`](crate::clue) macro, which uses the same syntax for the direction and text.
///
/// ```
/// use puzzled::crossword::{clue_spec, ClueSpec, Direction::*};
///
/// assert_eq!(clue_spec!(A: "A clue"), ClueSpec::new(Across, "A clue".to_string()));
/// assert_eq!(clue_spec!(D: "D clue"), ClueSpec::new(Down, "D clue".to_string()));
///
/// assert_eq!(clue_spec!(A: 'L').text(), &"L".to_string());
///
/// // Invalid syntax
/// // clue_spec(A: A clue)
/// // clue_spec(B: "B clue")
/// ```
#[macro_export]
macro_rules! clue_spec {
    ($dir:ident : $clue:literal) => {
        $crate::ClueSpec::new($crate::__dir!($dir), $clue)
    };

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid:tt)*, "clue_spec!")
    }};
}

/// Macro for constructing a [clue](crate::Clue) inline
/// A specification is constructed as
/// ```bnf
/// <clue> ::= <num> <spec> @ (<row>, <col>) + <len>
/// <num>  ::= `u8`
/// <spec> ::= <dir> : "<text>"
/// <row>  ::= `usize`
/// <col>  ::= `usize`
/// <len>  ::= `u8`
/// <dir>  ::= A | D
/// ```
/// Note that `<spec>` is just the [`clue_spec!`] syntax for constructing a [`ClueSpec`](crate::ClueSpec).
/// To place the clue, you also need
/// - The `<num>` that represents the order of all clues of the same [direction](crate::Direction), e.g. `1 D` denotes the first down clue, so `<num> == 1`.
/// - The `<row>` and `<col>` where the starting [square](crate::Square) of the clue is [positioned](crate::Position)
/// - A `<len>` to denote how many squares the clue takes up in the [`Squares`](crate::Squares) grid
/// ```
/// use puzzled::crossword::{clue, clue_spec, Clue, Position};
///
/// let clue = clue!(1 A: "Across clue" @ (1, 2) + 3);
///
/// assert_eq!(clue.num(), 1);
/// assert_eq!(clue.spec(), clue_spec!(A: "Across clue"));
/// assert_eq!(clue.start(), Position::new(1, 2));
/// assert_eq!(clue.len(), 3);
/// ```
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

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid:tt)*, "clue!")
    }};
}

/// Macro for constructing a [cell](crate::Cell) inline
///
/// Squares can be constructed a 2 different ways
/// 1. A [`char`] constructs a [letter](crate::Solution::Letter) cell
/// 2. A [`str`] constructs a [rebus](crate::Solution::Rebus) cell
///
/// Note that a [`square!`](crate::square) can be constructed with the same syntax
/// ```
/// use puzzled::crossword::{cell, Cell, Solution::*};
///
/// assert_eq!(cell!('L'), Cell::new(Letter('L')));
/// assert_eq!(cell!("ABC"), Cell::new(Rebus("ABC".to_string())));
///
/// // Invalid syntax
/// // cell!(9)
/// // cell!(L)
/// // cell!(abc)
/// ```
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

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid:tt)*, "cell!")
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __error {
    ($($tt:tt)*, $macro:literal) => {
        compile_error!(concat!(
            "Invalid syntax for '",
            $macro,
            "' found, please read its documentation to see how to construct it"
        ));
    };
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
