#[macro_export]
macro_rules! nonogram {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Colors
        $(- $color_id:tt : $color:tt )*
    ) => {{
        // Add fills and use them to construct the rules
        let grid = $crate::grid![
            [$( $crate::fill!($x0) ),+]
            $(, [$( $crate::fill!($x) ),+] )*
        ];

        let fills = $crate::Fills::new(grid);
        let rules = $crate::Rules::from_fills(&fills);

        // Add colors
        let colors = vec![];

        $crate::Nonogram::empty_from_rules(rules, colors).expect("Size should be small enough")
    }};
}

#[macro_export]
macro_rules! fill {
    () => {
        $crate::Fill::Blank
    };

    ($x:tt) => {{
        const F: $crate::Fill = $crate::Fill::from_str_const(stringify!($x));
        F
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __error {
    ($($tt:tt)*, $macro:literal) => {
        compile_error!(concat!(
            "Invalid syntax for '",
            $macro,
            "' found, please read its documentation to see how to construct it"
        ));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn nonogram() {
        use crate::nonogram;

        let nonogram = nonogram!(
            [1 . 1 . 1]
            [1 . . . 9]
            [1 . . . 1]
        );
    }
}
