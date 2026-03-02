/// Macro for constructing a [`Bit`](crate::Bit) inline
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! skyscraper {
    () => {
        None
    };

    (-) => {
        None
    };

    ($height:literal) => {
        Some($crate::Skyscraper::new($height))
    };

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid)*, "skyscraper!")
    }};
}
