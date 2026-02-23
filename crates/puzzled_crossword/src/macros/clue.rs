/// Macro for constructing a [`ClueSpec`](crate::ClueSpec) inline
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
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! clue_spec {
    ($dir:ident : $clue:literal) => {
        $crate::ClueSpec::new($crate::__dir!($dir), $clue)
    };

    ($($invalid:tt)*) => {{
        $crate::macro_error($($invalid)*, "clue_spec!")
    }};
}

/// Macro for constructing a [`Clue`](crate::Clue) inline
///
/// A clue is constructed as
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
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
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
        $crate::macro_error($($invalid)*, "clue!")
    }};
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
