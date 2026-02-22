#[macro_export]
macro_rules! nonogram {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Colors
        $(- $color_id:tt : $color:tt )*

        // Metadata
        $( $meta_key:ident : $meta_val:literal )*
    ) => {{
        // Add fills and use them to construct the rules
        let grid = $crate::grid![
            [$( $crate::fill!($x0) ),+]
            $(, [$( $crate::fill!($x) ),+] )*
        ];

        let fills = $crate::Fills::new(grid);
        let rules = $crate::Rules::from_fills(&fills);

        // Add colors
        let colors = $crate::Colors::default();

        // Add metadata
        #[allow(unused_mut)]
        let mut meta = $crate::Metadata::default();
        $(
            $crate::metadata!(meta, $meta_key : $meta_val);
        )*

        $crate::Nonogram::new_empty(rules, colors, meta).expect("Size should be small enough")
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

        assert_eq!(nonogram.rows(), 3);
        assert_eq!(nonogram.cols(), 5);
    }
}
