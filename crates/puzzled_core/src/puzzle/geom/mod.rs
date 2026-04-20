mod direction;
mod grid;
mod lattice;
mod line;
mod offset;
mod order;
mod position;
mod side;
mod size;

pub use direction::*;
pub use grid::*;
pub use lattice::*;
pub use line::*;
pub use offset::*;
pub use order::*;
pub use position::*;
pub use side::*;
pub use size::*;

pub(crate) fn clamped_add(lhs: usize, rhs: isize) -> usize {
    (lhs as isize).saturating_add(rhs).clamp(0, isize::MAX) as usize
}

pub trait Algerbraic {
    fn to_algebraic(&self) -> String;
}

impl Algerbraic for usize {
    fn to_algebraic(&self) -> String {
        let mut num = *self;
        let mut letters = String::new();

        loop {
            let rem = (10 + num % 26) as u32;
            let letter = char::from_digit(rem, 36).expect("Valid digit");
            letters.push(letter);

            num /= 26;
            if num == 0 {
                break;
            }
        }

        letters
    }
}
