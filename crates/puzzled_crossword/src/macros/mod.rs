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
    use puzzled_core::CellStyle;
    use rstest::rstest;

    use crate::{
        CrosswordCell,
        Solution::{self, *},
        square,
    };

    const _E: CellStyle = CellStyle::empty();
    const _I: CellStyle = CellStyle::INCORRECT;
    const _P: CellStyle = CellStyle::PREVIOUSLY_INCORRECT;
    const _R: CellStyle = CellStyle::REVEALED;
    const _C: CellStyle = CellStyle::CIRCLED;

    #[rstest]
    #[case(square!(A), Letter('A'), None, _E)]
    #[case(square!(A (A)), Letter('A'), Some(Letter('A')), _E)]
    #[case(square!(A (E)), Letter('A'), Some(Letter('E')), _I)]
    #[case(square!(A@), Letter('A'), None, _C)]
    #[case(square!(A*), Letter('A'), None, _R)]
    fn test_cell(
        #[case] square: Option<CrosswordCell>,
        #[case] solution: Solution,
        #[case] entry: Option<Solution>,
        #[case] style: CellStyle,
    ) {
        match square {
            Some(cell) => {
                assert_eq!(cell.solution(), &solution);
                assert_eq!(cell.entry(), entry.as_ref());
                assert_eq!(cell.style(), style);
            }
            _ => unreachable!("No test cases produce an empty square"),
        }
    }
}
