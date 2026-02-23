/// Macro for constructing a [`Square`](crate::Square) inline
///
/// Squares can be constructed in 3 different ways:
/// 1. Empty or `.` constructs a [black square](crate::Square::Black)
/// 2. A [`char`] constructs a [white square](crate::Square::White) with a [letter](crate::Solution::Letter) [cell](crate::Cell)
/// 3. A [`str`] constructs a white square with a [rebus](crate::Solution::Rebus) cell
///
/// Note that for the latter two, the syntax is analoguous to using [`cell!`](crate::cell).
/// ```
/// use puzzled::crossword::{square, CrosswordCell, cell, Solution, Solution::*};
///
/// assert_eq!(square!(), None::<CrosswordCell>);
/// assert_eq!(square!(.), None::<CrosswordCell>);
///
/// let make_cell = |sol: Solution| {
///     CrosswordCell::new(cell!(sol))
/// };
///
/// assert_eq!(square!('L'), Some(make_cell(Letter('L'))));
/// assert_eq!(square!("Rebus"), Some(make_cell(Rebus("Rebus".to_string()))));
///
/// // Invalid syntax
/// // square!(9)
/// // square!(L)
/// // square!(ABC)
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! square {
    // Empty squares
    () => {
        None
    };
    (.) => {
        None
    };

    // Cells
    ($sol:tt $($style:tt)+ ($entry:expr)) => {{
        let solution = $crate::__solution(stringify!($sol));
        let entry = $crate::__solution(stringify!($entry));

        let cell = $crate::cell!(solution $($style)+ (entry));
        Some($crate::CrosswordCell::new(cell))
    }};

    ($sol:tt ($entry:expr)) => {{
        let solution = $crate::__solution(stringify!($sol));
        let entry = $crate::__solution(stringify!($entry));

        let cell = $crate::cell!(solution (entry));
        Some($crate::CrosswordCell::new(cell))
    }};

    ($sol:tt $($rest:tt)*) => {{
        let solution = $crate::__solution(stringify!($sol));

        let cell = $crate::cell!(solution $($rest)*);
        Some($crate::CrosswordCell::new(cell))
    }};
}
