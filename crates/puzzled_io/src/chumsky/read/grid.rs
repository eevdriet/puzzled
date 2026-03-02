use std::fmt;

use chumsky::{
    IterParser, Parser,
    error::EmptyErr,
    extra::Err,
    prelude::{group, just},
};
use puzzled_core::{Grid, SidedGrid};

pub fn grid<'a, T, P>(value: P) -> impl Parser<'a, &'a str, Grid<T>, Err<EmptyErr>>
where
    T: fmt::Debug,
    P: Parser<'a, &'a str, T, Err<EmptyErr>> + Clone,
{
    grid_row(value)
        .padded()
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .try_map(|rows, _span| {
            let col_count = rows.first().map(|r| r.len()).unwrap_or(0);
            eprintln!("Rows: {rows:?}");
            eprintln!("Col count: {col_count}");
            let flat = rows.into_iter().flatten().collect();

            Grid::from_vec(flat, col_count).map_err(|_| EmptyErr::default())
        })
}

pub fn sided_grid<'a, T, U, V, S>(
    value: V,
    side: S,
) -> impl Parser<'a, &'a str, SidedGrid<T, U>, Err<EmptyErr>>
where
    T: fmt::Debug,
    V: Parser<'a, &'a str, T> + Clone,
    S: Parser<'a, &'a str, U> + Clone,
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
    .try_map(|(top, rows, bottom), _span| {
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
        let col_count = rows.first().map(|row| row.len()).unwrap_or(0);

        if !rows.iter().all(|row| row.len() == col_count) {
            return Err(EmptyErr::default());
            // return Err(Rich::custom(span, ())); // Use Rich error type
        }

        for side in [top.as_ref(), right.as_ref(), bottom.as_ref(), left.as_ref()] {
            if side.is_some_and(|s| s.len() != col_count) {
                return Err(EmptyErr::default());
            }
        }

        // Create the grid and resulting sided grid
        let flat = rows.into_iter().flatten().collect();
        let grid = Grid::from_vec(flat, col_count).expect("Verified column count");

        Ok(SidedGrid {
            grid,
            top,
            bottom,
            left,
            right,
        })
    })
}

pub fn grid_row<'a, T, P>(value: P) -> impl Parser<'a, &'a str, Vec<T>, Err<EmptyErr>> + Clone
where
    T: fmt::Debug,
    P: Parser<'a, &'a str, T, Err<EmptyErr>> + Clone,
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
        let value = text::digits::<_, Err<EmptyErr>>(10)
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
        let value = text::digits::<_, Err<EmptyErr>>(10)
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
