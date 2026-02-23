#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! nonogram {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Colors
        $(- $color_id:tt : $color:literal )*

        // Metadata
        $( $meta_key:ident : $meta_value:expr )*
    ) => {{
        // Add fills and use them to construct the rules
        let grid = $crate::grid![
            [$( $crate::fill!($x0) ),+]
            $(, [$( $crate::fill!($x) ),+] )*
        ];

        let fills = $crate::Fills::new(grid);
        let rules = $crate::Rules::from_fills(&fills);

        // Add colors
        let mut colors = $crate::Colors::default();
        $(
            let fill = $crate::fill!($color_id);
            let color = $crate::color!($color);

            colors.insert_fill(fill, color);
        )*

        // Add metadata
        let meta = $crate::metadata!( $( $meta_key:ident : $meta_value:expr),*);

        // Create puzzle
        $crate::Nonogram::new_empty(rules, colors, meta).expect("Size should be small enough")
    }};
}
