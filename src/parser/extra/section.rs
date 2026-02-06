use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use crate::ExtrasError;

pub enum ExtraSection {
    Grbs,
    Rtbl,
    Ltim,
    Gext,
}

impl ExtraSection {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ExtraSection::Grbs => "GRBS",
            ExtraSection::Rtbl => "RTBL",
            ExtraSection::Ltim => "LTIM",
            ExtraSection::Gext => "GEXT",
        }
    }
}

impl FromStr for ExtraSection {
    type Err = ExtrasError;

    fn from_str(section: &str) -> std::result::Result<Self, Self::Err> {
        match section {
            "GRBS" => Ok(ExtraSection::Grbs),
            "RTBL" => Ok(ExtraSection::Rtbl),
            "LTIM" => Ok(ExtraSection::Ltim),
            "GEXT" => Ok(ExtraSection::Gext),

            // Found invalid section
            _ => Err(ExtrasError::InvalidSection {
                found: section.to_string(),
            }),
        }
    }
}

impl Debug for ExtraSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
