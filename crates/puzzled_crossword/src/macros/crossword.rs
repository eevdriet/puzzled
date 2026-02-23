/// Macro for constructing a [`Crossword`](crate::Crossword) inline
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
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! crossword {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Clue definitions
        $(- $dir:ident : $clue:literal )*

        // Metadata
        $( $meta_key:ident : $meta_value:expr )*
    ) => {{
        // Add squares
        let grid = $crate::grid![
            [$( $crate::square!($x0) ),+]
            $(, [$( $crate::square!($x) ),+] )*
        ];
        let squares = $crate::Squares::new(grid);

        // Add clues
        let clues = vec![$($crate::clue_spec!($dir : $clue)),*];

        // Add metadata
        let meta = $crate::metadata!( $( $meta_key:ident : $meta_value:expr),*);

        // Create puzzle
        let mut puzzle = $crate::Crossword::from_squares(squares, meta);
        puzzle.insert_clues(clues);

        puzzle
    }};

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid)*, "crossword!")
    }};
}
