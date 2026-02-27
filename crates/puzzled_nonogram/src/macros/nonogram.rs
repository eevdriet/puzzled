#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! nonogram {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Rules
        $($line:ident $num:literal : [$($count:literal $fill:tt),+])*

     // R 1: [1 a, 2 b]
     // R 2: [2 b, 1 a]
     // R 3: [1 a]

        // Colors
        $(- $color_id:tt : $color:literal )*

        // Metadata
        $( $meta_key:ident : $meta_value:expr )*
    ) => {{
        // Create fills grid and count the number of columns
        let fills = $crate::grid![
            [$( $crate::fill!($x0) ),+]
            $(, [$( $crate::fill!($x) ),+] )*
        ];
        let fills = fills.map(|b| $crate::Cell::new(b));

        // Create the rules

        // Create the colors colors
        #[allow(unused_mut)]
        let mut colors = $crate::Colors::default();
        $(
            if let Some(fill) = $crate::fill!($color_id) {
                let color = $crate::color!($color);

                colors.insert(fill, color);
            }
        )*

        // Add metadata
        let meta = $crate::metadata!($( $meta_key : $meta_value),*);

        // Create puzzle
        $crate::Nonogram::new(fills, colors, meta)
    }};
}
