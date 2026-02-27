#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! fill {
    () => {
        None
    };
    (-) => {
        None
    };

    (x) => {
        Some($crate::Fill::Cross)
    };
    (X) => {
        Some($crate::Fill::Cross)
    };

    ($x:tt) => {{
        const F: $crate::Fill = match $crate::Fill::decode_str($crate::smart_stringify!($x)) {
            Ok(f) => f,
            Err(_) => panic!(concat!("Invalid fill literal '", stringify!($x), "'"))
        };

        Some(F)
    }};

    ($($invalid:tt)*) => {{
        $crate::macro_error($($invalid)*, "fill!")
    }};
}
