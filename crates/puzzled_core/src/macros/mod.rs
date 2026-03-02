mod cell;
mod color;
mod grid;
mod lattice;
mod line;
mod metadata;

#[doc(hidden)]
#[macro_export]
macro_rules! __count {
    () => (0usize);
    ( $first:tt $($rest:tt)* ) => (1usize + $crate::__count!($($rest)*));
}

#[doc(hidden)]
#[macro_export]
macro_rules! macro_error {
    ($($tt:tt)*, $macro:literal) => {
        compile_error!(concat!(
            "Invalid syntax for '",
            $macro,
            "' found (",
            stringify!($($tt)*),
            "), please read its documentation to see how to construct it"
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! smart_stringify {
    ($x:tt) => {{
        let s = stringify!($x);

        match s.as_bytes() {
            [b'"', b'"', contents @ .., b'"', b'"'] => match std::str::from_utf8(contents) {
                Ok(s) => s,
                Err(_) => unreachable!(),
            },
            _ => s,
        }
    }};
}

// #[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
// #[macro_export]
// macro_rules! __skycrapers_impl {
//     (
//         [$tl:ident; $($row0:tt),+ ;+]
//         $(; [$($row:tt),+])*
//         [+; $($row1:tt),+ ;+]
//     ) => {{
//         // Count columns and rows and make sure the puzzle is square
//         let mut _assert_width0 = [(); $crate::__count!($($row0)+)];
//         const COLS: usize = $crate::__count!($($row0)+);
//         const ROWS: usize = 2 + $crate::__count!($([$($row),+])*);
//
//         $(
//             _assert_width0 = [(); $crate::__count!($($row)+)];
//         )*
//
//         _assert_width0 = [(); $crate::__count!($($row1)+)];
//         let _: [(); COLS] = [(); ROWS];
//
//         // // Construct cell grid
//         // let grid = $crate::grid![
//         //     $([$($crate::cell!($rows)),+]),*
//         // ];
//         // let cells = $crate::Cells::new(grid);
//         //
//         // // Add clues from the first and last lines
//         // let mut clues = $crate::Clues::default();
//         // let meta = $crate::Metadata::default();
//         //
//         // $crate::Skyscrapers::new(cells, clues, meta)
//         0
//     }};
//
//     // Columns
//     (@cols $clues:ident; $col:expr; $num:tt $(, $nums:tt)*) => {
//         $crate::skyscrapers!(@cell $clues; $crate::Line::Col($col); $num);
//         $crate::skyscrapers!(@cols $clues; (1usize + $col); $($nums,)*);
//     };
//     (@cols $clues:ident; $col:expr;) => {};
//
//     // Rows
//     (@rows $clues:ident; $row:expr; $first:tt $(, $nums:tt)*, $last:tt) => {
//
//     };
//
//     // Cell
//     (@cell $clues:ident; $line:expr; $dir:expr; -) => {};
//     (@cell $clues:ident; $line:expr; $dir:expr; |) => {};
//
//     (@cell $clues:ident; $line:expr; $dir:expr; $num:expr) => {
//         $clues.insert($crate::ClueId::new($line, $dir), $num);
//     };
// }
