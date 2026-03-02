/// Macro for constructing a [`Binario`](crate::Binario) inline
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! binario {
    (
        // Grid definition
        [$($x0:tt)+] $( [$($x:tt)+])*

        // Metadata
        $( $meta_key:ident : $meta_value:literal )*
    ) => {{
        // Add bits
        let bits = $crate::grid![
            [$( $crate::bit!($x0) ),+]
            $(, [$( $crate::bit!($x) ),+] )*
        ];
        let bits = bits.map(|b| $crate::Cell::new(b));

        // Add metadata
        let meta = $crate::metadata!($( $meta_key : $meta_value),*);

        // Create puzzle
        $crate::Binario::new(bits, meta)
    }};

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid)*, "binario!")
    }};
}
