use nom::{
    IResult, Parser,
    character::{
        char,
        complete::{multispace0, newline, space0},
    },
    combinator::opt,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, terminated},
};
use puzzled_core::{Grid, SidedGrid};

pub fn grid<'a, T, P>(input: &'a str, value: P) -> IResult<&'a str, Grid<T>>
where
    P: Parser<&'a str, Output = T, Error = Error<&'a str>>,
{
    // A row = [ cell cell cell ... ]
    let parse_row = delimited(char('['), separated_list1(space0, value), char(']'));

    // All rows separated by newline
    let (input, rows) = delimited(
        multispace0,
        separated_list1(newline, parse_row),
        multispace0,
    )
    .parse(input)?;

    let col_count = rows.first().map(|r| r.len()).unwrap_or(0);

    // Ensure rectangular
    if !rows.iter().all(|row| row.len() == col_count) {
        return Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    let flat = rows.into_iter().flatten().collect();
    let grid = Grid::from_vec(flat, col_count).expect("Verified column count");

    Ok((input, grid))
}

// pub fn sided_grid<'a, T, U, P1, P2>(
//     input: &'a str,
//     value: P1,
//     side: P2,
// ) -> IResult<&'a str, SidedGrid<T, U>>
// where
//     P1: Parser<&'a str, Output = T, Error = Error<&'a str>> + Clone,
//     P2: Parser<&'a str, Output = U, Error = Error<&'a str>> + Clone,
// {
//     // A row = [ cell cell cell ... ]
//     let side_row = || opt(separated_list1(space0, side.clone()));
//     let grid_row = (
//         opt(side.clone()),
//         separated_list1(space0, value),
//         opt(side.clone()),
//     );
//
//     // All rows separated by newline
//     let (input, top) = terminated(side_row(), newline).parse(input)?;
//     let (input, rows) =
//         delimited(multispace0, separated_list1(newline, grid_row), multispace0).parse(input)?;
//     let (input, bottom) = side_row().parse(input)?;
//
//     let col_count = top.len();
//
//     // Ensure the grid and top/bottom rows have an equal amount of values
//     if col_count != bottom.len() {
//         return Err(nom::Err::Failure(nom::error::Error::new(
//             input,
//             nom::error::ErrorKind::Verify,
//         )));
//     }
//
//     if !rows.iter().all(|(_, row, _)| row.len() == col_count) {
//         return Err(nom::Err::Failure(nom::error::Error::new(
//             input,
//             nom::error::ErrorKind::Verify,
//         )));
//     }
//
//     let (left, data, right) = rows.into_iter().fold(
//         (None, vec![], None),
//         |(mut left, mut data, mut right), (opt_l, d, opt_r)| {
//             let left_items = left.unwrap_or_default();
//             let left = opt_l.is_some().then_some(left);
//             let ll = match (left, opt_l) {
//                 (_, None) => None,
//                 (None, Some(l)) => Some(vec![l]),
//                 (Some(items), Some(l)) => {
//                     items.push(l);
//                     Some(items)
//                 }
//             };
//
//             data.push(d);
//             right.push(r);
//
//             (ll, data, right)
//         },
//     );
//
//     let flat = data.into_iter().flatten().collect();
//     let grid = Grid::from_vec(flat, col_count).expect("Verified column count");
//
//     Ok((
//         input,
//         SidedGrid {
//             grid,
//             top,
//             left,
//             right,
//             bottom,
//         },
//     ))
// }
