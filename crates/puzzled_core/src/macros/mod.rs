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
