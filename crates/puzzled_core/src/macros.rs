#[doc(hidden)]
#[macro_export]
macro_rules! __count {
    () => (0usize);
    ( $first:tt $($rest:tt)* ) => (1usize + $crate::__count!($($rest)*));
}

#[macro_export]
macro_rules! grid {
    (
        [$($x0:expr),+ $(,)?]
        $(, [$($x:expr),+ $(,)?] $(,)?)*
    ) => {{
        // Count columns
        let mut _assert_width0 = [(); $crate::__count!($($x0)+)];
        let cols = $crate::__count!($($x0)+);
        let rows = 1usize;

        $(
            let _assert_width = [(); $crate::__count!($($x)+)];
            _assert_width0 = _assert_width;
            let rows = rows + 1usize;
        )*

        let mut vec = Vec::with_capacity(rows.checked_mul(cols).unwrap());

        $( vec.push($x0); )*
        $( $( vec.push($x); )* )*

        $crate::Grid::from_vec(vec, cols).unwrap()
    }};
}

#[macro_export]
macro_rules! squares {
    (
        [$($x0:tt),+ $(,)?] $(, [$($x:tt),+ $(,)?] $(,)?)*
    ) => {{
        let grid = $crate::grid![
            [$( $crate::square!($x0) ),+]
            $(, [$( $crate::square!($x) ),+] )*
        ];

        $crate::OptGrid::new(grid)
    }};
}
