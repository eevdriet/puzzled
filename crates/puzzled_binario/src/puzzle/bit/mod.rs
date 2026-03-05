mod bits;

use std::{fmt, ops::Not, str::FromStr};

pub use bits::*;
use puzzled_core::Color;

#[derive(Debug, thiserror::Error)]
pub enum BitError {
    #[error("Tried to construct from {0}, only 0/1 allowed")]
    Overflow(u8),

    #[error("Cannot construct bit from {0:?}, only #000 (zero) or #FFF (one) allowed")]
    InvalidColor(Color),

    #[error("Cannot construct bit from {0:?}, only 0, 1, t, f, true and false allowed")]
    InvalidText(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bit {
    #[default]
    Zero,

    One,
}

impl Bit {
    pub fn is_zero(&self) -> bool {
        matches!(self, Bit::Zero)
    }
    pub fn is_one(&self) -> bool {
        matches!(self, Bit::One)
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bit::Zero => 0,
                Bit::One => 1,
            },
        )
    }
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

impl FromStr for Bit {
    type Err = BitError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "0" | "f" | "false" => Ok(Bit::Zero),
            "1" | "t" | "true" => Ok(Bit::One),
            other => Err(BitError::InvalidText(other.to_string())),
        }
    }
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}
