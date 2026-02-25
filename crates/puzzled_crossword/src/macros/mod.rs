mod clue;
mod crossword;
mod square;

// Trait for converting anything into Solution
pub(crate) trait __IntoSolution {
    fn into_solution(self) -> crate::Solution;
}

impl __IntoSolution for char {
    fn into_solution(self) -> crate::Solution {
        crate::Solution::Letter(self)
    }
}

impl __IntoSolution for String {
    fn into_solution(self) -> crate::Solution {
        crate::Solution::Rebus(self)
    }
}

#[doc(hidden)]
pub fn __prepare(s: &str) -> String {
    s.trim_matches('"').trim_matches('\'').to_ascii_uppercase()
}

#[doc(hidden)]
pub fn __solution(sol_str: &str) -> crate::Solution {
    let sol_str = __prepare(sol_str);

    if sol_str.len() == 1 {
        sol_str
            .chars()
            .next()
            .expect("Verified length")
            .into_solution()
    } else {
        sol_str.into_solution()
    }
}

#[cfg(all(test, feature = "macros"))]
mod tests {
    use puzzled_core::{Cell, CellStyle, Square};
    use rstest::rstest;

    use crate::{
        Solution::{self, *},
        square,
    };

    type CrosswordCell = Cell<Solution>;

    const _E: CellStyle = CellStyle::empty();
    const _I: CellStyle = CellStyle::INCORRECT;
    const _P: CellStyle = CellStyle::PREVIOUSLY_INCORRECT;
    const _R: CellStyle = CellStyle::REVEALED;
    const _C: CellStyle = CellStyle::CIRCLED;

    #[rstest]
    #[case(square!(A), Letter('A'), _E)]
    #[case(square!(A@), Letter('A'), _C)]
    #[case(square!(A*), Letter('A'), _R)]
    fn test_cell(
        #[case] square: Square<CrosswordCell>,
        #[case] solution: Solution,
        #[case] style: CellStyle,
    ) {
        match square.inner() {
            Some(cell) => {
                assert_eq!(cell.solution.clone().unwrap(), solution);
                assert_eq!(cell.style, style);
            }
            _ => unreachable!("No test cases produce an empty square"),
        }
    }
}
