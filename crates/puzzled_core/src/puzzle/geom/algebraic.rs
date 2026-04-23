use crate::Position;

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

impl Algerbraic for Position {
    fn to_algebraic(&self) -> String {
        format!("{}{}", self.col.to_algebraic(), self.row)
    }
}
