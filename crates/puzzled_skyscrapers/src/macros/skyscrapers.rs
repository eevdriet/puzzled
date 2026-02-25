/*
    [+ - - 3 - - +]
    [2 . . . . . 2]
    [3 . . . . . |]
    [| . . . . . |]
    [| . . . . . 1]
    [| . . . . . 2]
    [+ 2 - - - - +]
*/

// (
//     // Grid definition
//     [$($x0:tt)+] $( [$($x:tt)+])*
//
//     // Clue definitions
//     $(- $dir:ident : $clue:literal )*
//
//     // Metadata
//     $( $meta_key:ident : $meta_value:literal )*
// ) => {{
//     // Add squares
//     let grid = $crate::grid![
//         [$( $crate::square!($x0) ),+]
//         $(, [$( $crate::square!($x) ),+] )*
//     ];
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! skycrapers {
    (
        [+ $($top:tt)* +]

        $(
            [| $($row:tt)+ |]
        )+

        [+ $($bottom:tt)+ +]
    ) => {{

        const COLS: usize = $crate::__count!($($top)+);

        const ROWS: usize = $crate::__count!(
            $( [$($row)+] )+
        );

        const _: [(); COLS] = [(); ROWS];

        $crate::Grid::from([
            $(
                $crate::skycrapers!(@row COLS; $($row)+)
            ),+
        ])

    }};
}

#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! __skycrapers_impl {
    (
        [+; $($row0:tt),+ ;+]
        $(; [$($row:tt),+])*
        [+; $($row1:tt),+ ;+]
    ) => {{
        // Count columns and rows and make sure the puzzle is square
        let mut _assert_width0 = [(); $crate::__count!($($row0)+)];
        const COLS: usize = $crate::__count!($($row0)+);
        const ROWS: usize = 2 + $crate::__count!($([$($row),+])*);

        $(
            _assert_width0 = [(); $crate::__count!($($row)+)];
        )*

        _assert_width0 = [(); $crate::__count!($($row1)+)];
        let _: [(); COLS] = [(); ROWS];

        // // Construct cell grid
        // let grid = $crate::grid![
        //     $([$($crate::cell!($rows)),+]),*
        // ];
        // let cells = $crate::Cells::new(grid);
        //
        // // Add clues from the first and last lines
        // let mut clues = $crate::Clues::default();
        // let meta = $crate::Metadata::default();
        //
        // $crate::Skyscrapers::new(cells, clues, meta)
        0
    }};

    // Columns
    (@cols $clues:ident; $col:expr; $num:tt $(, $nums:tt)*) => {
        $crate::skyscrapers!(@cell $clues; $crate::Line::Col($col); $num);
        $crate::skyscrapers!(@cols $clues; (1usize + $col); $($nums,)*);
    };
    (@cols $clues:ident; $col:expr;) => {};

    // Rows
    (@rows $clues:ident; $row:expr; $first:tt $(, $nums:tt)*, $last:tt) => {

    };

    // Cell
    (@cell $clues:ident; $line:expr; $dir:expr; -) => {};
    (@cell $clues:ident; $line:expr; $dir:expr; |) => {};

    (@cell $clues:ident; $line:expr; $dir:expr; $num:expr) => {
        $clues.insert($crate::ClueId::new($line, $dir), $num);
    };
}
