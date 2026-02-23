#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
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
