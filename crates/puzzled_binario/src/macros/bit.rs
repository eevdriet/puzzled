/// Macro for constructing a [`Bit`](crate::Bit) inline
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! bit {
    () => {
        Cell::default()
    };

    (_) => {
        None
    };

    (0) => {
        Some($crate::Bit::Zero)
    };
    (1) => {
        Some($crate::Bit::One)
    };
    (false) => {
        Some($crate::Bit::Zero)
    };
    (true) => {
        Some($crate::Bit::One)
    };

    ($($invalid:tt)*) => {
        compile_error!(concat!(
            "Invalid Bit representation '",
            stringify!($($invalid)*),
            "', use one of: 0, 1, true, false"
        ));
    };
}
