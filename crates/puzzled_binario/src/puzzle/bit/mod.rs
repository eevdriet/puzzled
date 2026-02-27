mod bits;

pub use bits::*;
use puzzled_core::Color;

#[derive(Debug, thiserror::Error)]
pub enum BitError {
    #[error("Tried to construct from {0}, only 0/1 allowed")]
    Overflow(u8),

    #[error("Cannot construct bit from {0:?}, only #000 (zero) or #FFF (one) allowed")]
    InvalidColor(Color),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bit {
    #[default]
    Zero,

    One,
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        match value {
            false => Bit::Zero,
            true => Bit::One,
        }
    }
}

impl From<Bit> for bool {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl From<Bit> for u8 {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl TryFrom<u8> for Bit {
    type Error = BitError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bit::Zero),
            1 => Ok(Bit::Zero),
            bit => Err(BitError::Overflow(bit)),
        }
    }
}
