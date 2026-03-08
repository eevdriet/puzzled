use std::fmt;

use chumsky::{
    IterParser, Parser,
    extra::Err,
    prelude::{group, just},
};
use puzzled_core::{Grid, GridError, SidedGrid};

use crate::text::read::ParseError;

pub fn grid<'a, T, P>(value: P) -> impl Parser<'a, &'a str, Grid<T>, Err<ParseError<'a>>>
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    grid_row(value)
        .padded()
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .try_map(|rows, span| {
            let col_count = rows.first().map(|r| r.len()).unwrap_or(0);
            let flat = rows.into_iter().flatten().collect();

            Grid::from_vec(flat, col_count).map_err(|err| ParseError::custom(span, err.to_string()))
        })
}

pub fn sided_grid<'a, T, U, V, S>(
    value: V,
    side: S,
) -> impl Parser<'a, &'a str, SidedGrid<T, U>, Err<ParseError<'a>>>
where
    T: fmt::Debug,
    V: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
    S: Parser<'a, &'a str, U, Err<ParseError<'a>>> + Clone,
{
    let side_row = side
        .clone()
        .repeated()
        .collect::<Vec<_>>()
        .padded()
        .or_not();

    let row = group((
        side.clone().or_not(),
        grid_row(value),
        side.clone().or_not(),
    ));

    group((
        side_row.clone(),
        row.repeated().collect::<Vec<_>>().padded(),
        side_row,
    ))
    .try_map(|(top, rows, bottom), span| {
        // Separate sided rows into left, right and the rows
        let (left, rows, right) = rows.into_iter().fold(
            (None::<Vec<U>>, vec![], None::<Vec<U>>),
            |(mut left, mut data, mut right), (opt_l, row, opt_r)| {
                some_or_extend(&mut left, opt_l);
                some_or_extend(&mut right, opt_r);
                data.push(row);

                (left, data, right)
            },
        );

        // Verify that every row and side of the grid have the same length
        let cols = rows.first().map(|row| row.len()).unwrap_or(0);

        for (side_str, side) in [("top", top.as_ref()), ("bottom", bottom.as_ref())] {
            if let Some(side) = side
                && side.len() != cols
            {
                let err = GridError::InvalidSide {
                    side: side_str.to_string(),
                    found: side.len(),
                    expected: rows.len(),
                };

                return Err(ParseError::custom(span, err.to_string()));
            }
        }

        for (side_str, side) in [("left", left.as_ref()), ("right", right.as_ref())] {
            if let Some(side) = side
                && side.len() != rows.len()
            {
                let err = GridError::InvalidSide {
                    side: side_str.to_string(),
                    found: side.len(),
                    expected: rows.len(),
                };

                return Err(ParseError::custom(span, err.to_string()));
            }
        }

        // Create the grid and resulting sided grid
        let flat = rows.into_iter().flatten().collect();
        let grid =
            Grid::from_vec(flat, cols).map_err(|err| ParseError::custom(span, err.to_string()))?;

        Ok(SidedGrid {
            grid,
            top,
            bottom,
            left,
            right,
        })
    })
}

pub fn grid_row<'a, T, P>(value: P) -> impl Parser<'a, &'a str, Vec<T>, Err<ParseError<'a>>> + Clone
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    value
        .padded()
        .repeated()
        .collect::<Vec<T>>()
        .padded()
        .delimited_by(just('['), just(']'))
}

fn some_or_extend<T>(opt_vec: &mut Option<Vec<T>>, opt_item: Option<T>) {
    // If the item is Some(T), insert into the Vec inside the Option
    if let Some(item) = opt_item {
        opt_vec.get_or_insert_with(Vec::new).push(item);
    }

    // If item_opt is None, do nothing (vec_opt remains as is)
}

#[cfg(test)]
mod tests {
    use chumsky::text;
    use rstest::rstest;

    use super::*;
    use puzzled_core::grid;

    #[rstest]
    #[case("[1 2 3 4 5]", vec![1, 2, 3, 4, 5])]
    #[case("[ 1  2  3 4  5]", vec![1, 2, 3, 4, 5])]
    fn test_grid_row(#[case] input: &str, #[case] output: Vec<usize>) {
        let value = text::digits::<_, Err<ParseError<'_>>>(10)
            .to_slice()
            .from_str()
            .unwrapped();

        let row: Vec<usize> = grid_row(value)
            .parse(input)
            .into_output()
            .expect("Parsing should succeed");

        assert_eq!(output, row);
    }

    #[rstest]
    #[case("[1 2]", grid![[1, 2]])]
    #[case("[ 1  2  3 4  5]", grid![[1, 2, 3, 4, 5]])]
    #[case("[1 2] [3 4]", grid![[1, 2], [3, 4]])]
    fn test_grid(#[case] input: &str, #[case] output: Grid<usize>) {
        let value = text::digits::<_, Err<ParseError<'_>>>(10)
            .to_slice()
            .from_str()
            .unwrapped();

        let grid: Grid<usize> = grid(value)
            .parse(input)
            .into_output()
            .expect("Parsing should succeed: {output}");

        assert_eq!(output, grid);
    }
}
