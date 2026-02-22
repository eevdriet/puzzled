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

#[macro_export]
macro_rules! metadata {
    ($meta:ident, author : $author:literal) => {
        $meta.author = Some($author.to_string());
    };

    ($meta:ident, copyright : $copyright:literal) => {
        $meta.copyright = Some($copyright.to_string());
    };

    ($meta:ident, notes : $notes:literal) => {
        $meta.notes = Some($val.to_string());
    };

    ($meta:ident, title : $title:literal) => {
        $meta.title = Some($title.to_string());
    };

    ($meta:ident, version : $version:literal) => {
        if let Ok(version) = $crate::Version::new($version.as_bytes()) {
            $meta.version = Some(version);
        }
    };

    ($meta:ident, timer : $timer:literal) => {
        if let Ok(timer) = $crate::Timer::from_str($timer) {
            $meta.timer = timer;
        }
    };

    ($meta:ident, $key:ident : $val:literal) => {
        compile_error!(concat!("Invalid meta property: ", stringify!($key)));
    };
}
