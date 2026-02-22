use std::fmt;

pub type ColorId = usize;

type ColorValue = u8;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub red: ColorValue,
    pub green: ColorValue,
    pub blue: ColorValue,
    pub alpha: Option<ColorValue>,
}

#[derive(Debug, thiserror::Error)]
#[error("Color error: {0}")]
pub enum Error {
    #[error("Invalid hex string '{found}' found: {reason}")]
    HexError { found: String, reason: String },
}

impl Color {
    pub const fn rgba(
        red: ColorValue,
        green: ColorValue,
        blue: ColorValue,
        alpha: ColorValue,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: Some(alpha),
        }
    }

    pub const fn rgb(red: ColorValue, green: ColorValue, blue: ColorValue) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: None,
        }
    }

    pub fn hex(hex: &str) -> Result<Self, Error> {
        let hex = hex.trim_start_matches('#');

        let parse_color = |len: usize| -> Result<Self, Error> {
            // Verify whether to read each byte once or twice (e.g. #0A1 => #00AA11)
            let should_double = len < 6;
            let size = if should_double { 1 } else { 2 };

            let mut iter = hex.as_bytes().chunks(size).map(|chunk| {
                let s = str::from_utf8(chunk).expect("Reconstructing valid str");

                // Make sure the string is radix-16
                match u8::from_str_radix(s, 16) {
                    Ok(mut val) => {
                        // Double byte if necessary
                        if should_double {
                            val *= 17;
                        }

                        Ok(val)
                    }
                    Err(_) => Err(Error::HexError {
                        found: hex.to_string(),
                        reason: "Invalid hex digit".into(),
                    }),
                }
            });

            // Construct color
            let red = iter.next().expect("Verified length")?;
            let green = iter.next().expect("Verified length")?;
            let blue = iter.next().expect("Verified length")?;
            let alpha = iter
                .next()
                .expect("Verified length")
                .unwrap_or(ColorValue::MAX);

            Ok(Self::rgba(red, green, blue, alpha))
        };

        match hex.len() {
            len @ (3 | 4 | 6 | 8) => parse_color(len),
            _ => Err(Error::HexError {
                found: hex.to_string(),
                reason: "Invalid hex length (expected 3, 4, 6 or 8)".into(),
            }),
        }
    }
}

impl Color {
    pub fn to_hex(&self) -> String {
        match self.alpha {
            Some(alpha) => format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue, alpha
            ),
            None => format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize, de};

    use crate::Color;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Color {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.to_hex().serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Color {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let hex = String::deserialize(deserializer)?;
            let color = Self::hex(&hex).map_err(de::Error::custom)?;

            Ok(color)
        }
    }
}
