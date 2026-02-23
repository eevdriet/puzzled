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
macro_rules! metadata {
    ( $( $key:ident : $value:expr),* $(,)? ) => {
        $crate::Metadata {
            $(
                $key: metadata!(@transform $key, $value),
            )*
            ..Default::default()
        }
    };

     // String parsing
    (@transform author, $value:literal) => {
        Some($value.into())
    };

    (@transform copyright, $value:literal) => {
        Some($value.into())
    };

    (@transform notes, $value:literal) => {
        Some($value.into())
    };

    (@transform title, $value:literal) => {
        Some($value.into())
    };

    // Version parsing
    (@transform version, $value:literal) => {
        match $crate::Version::new($value.as_bytes()) {
            Ok(v) => Some(v),
            Err(_) => panic!("Invalid version string"),
        }
    };

    // Timer parsing
    (@transform timer, $value:literal) => {
        match $crate::Timer::from_str($value) {
            Ok(t) => t,
            Err(_) => panic!("Invalid timer string"),
        }
    };

    // Unknown key
    (@transform $key:ident, $value:literal) => {
        compile_error!(concat!("Unknown metadata: ", stringify!($key)))
    };
}

/// Macro for constructing a [cell](crate::Cell) inline
///
/// Squares can be constructed a 2 different ways
/// 1. A [`char`] constructs a [letter](crate::Solution::Letter) cell
/// 2. A [`str`] constructs a [rebus](crate::Solution::Rebus) cell
///
/// Note that a [`square!`](crate::square) can be constructed with the same syntax
/// TODO: rewrite examples
#[macro_export]
macro_rules! cell {
// Rebus or Letter with optional entry and optional styles
    // Solution + styles + entry
    ($sol:tt $($style:tt)+ ($entry:expr)) => {{
        let style = $crate::style!($($style)+);

        // Create the cell from solution and style
        let cell = $crate::Cell::new_styled($sol, style);

        // Add entry
        cell.enter($entry);

        cell
    }};

    // Solution + entry
    ($sol:tt ($entry:expr)) => {{
        // Create the cell from solution
        let mut cell = $crate::Cell::new($sol);

        // Add entry
        cell.enter($entry);

        cell
    }};

    // Solution + styles
    ($sol:tt $($style:tt)+) => {{
        let style = $crate::style!($($style)+);

        // Create the cell from solution and style
        $crate::Cell::new_styled($sol, style)
    }};


    // Solution
    ($sol:expr) => {{
        $crate::Cell::new($sol)
    }};

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid)*, "cell!")
    }};
}

#[macro_export]
macro_rules! style {
    () => {
        $crate::CellStyle::default()
    };

    (~ $($rest:tt)*) => {
        $crate::CellStyle::PREVIOUSLY_INCORRECT
        | $crate::style!($($rest)*)
    };

    (* $($rest:tt)*) => {
        $crate::CellStyle::REVEALED
        | $crate::style!($($rest)*)
    };

    (@ $($rest:tt)*) => {
        $crate::CellStyle::CIRCLED
        | $crate::style!($($rest)*)
    };

    ($invalid:tt $($rest:tt)*) => {
        compile_error!(
            concat!(
                "Unknown style suffix: '",
                stringify!($invalid),
                "' (only ~, * and @ allowed)"
            )
        );
    };
}

#[macro_export]
macro_rules! color {
    // rgba(r, g, b, a)
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        $crate::Color::rgba($r, $g, $b, $a)
    };

    // rgb(r, g, b)
    ($r:expr, $g:expr, $b:expr) => {
        $crate::Color::rgb($r, $g, $b)
    };

    // hex string literal
    ($hex:literal) => {{
        match $crate::Color::hex($hex) {
            Ok(color) => color,
            Err(err) => panic!("{}", err),
        }
    }};

    // everything else → compile error
    ($($invalid:tt)*) => {
        compile_error!(concat!(
            "Invalid color '",
            stringify!($($invalid)*),
            "', use one of:\n\
             color!(r, g, b)\n\
             color!(r, g, b, a)\n\
             color!(\"#RRGGBB\" | \"#RGB\" | \"#RRGGBBAA\" | \"#RGBA\")"
        ));
    };
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
