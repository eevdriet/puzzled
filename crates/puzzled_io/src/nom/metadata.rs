use nom::{
    IResult, Parser,
    character::{
        char,
        complete::{alpha1, newline, space0},
    },
    multi::separated_list0,
    sequence::{delimited, separated_pair, terminated},
};
use puzzled_core::Metadata;

use crate::nom::quoted_string;

pub fn metadata(input: &str) -> IResult<&str, Metadata> {
    let meta_field = separated_pair(alpha1, delimited(space0, char(':'), space0), quoted_string);
    let (input, fields) = separated_list0(newline, terminated(meta_field, space0)).parse(input)?;

    let mut metadata = Metadata::default();

    for (prop, value) in fields {
        match prop.trim().to_ascii_lowercase().as_str() {
            "author" => {
                metadata = metadata.with_author(value.to_string());
            }
            "copyright" => {
                metadata = metadata.with_copyright(value.to_string());
            }
            "notes" => {
                metadata = metadata.with_notes(value.to_string());
            }
            "title" => {
                metadata = metadata.with_title(value.to_string());
            }
            _ => {}
        }
    }

    Ok((input, metadata))
}
