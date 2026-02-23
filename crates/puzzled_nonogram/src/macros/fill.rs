#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! fill {
    () => {
        $crate::Fill::Blank
    };

    ($x:tt) => {{
        const F: $crate::Fill = match $crate::Fill::decode_str(stringify!($x)) {
            Ok(f) => f,
            Err(_) => panic!(concat!("Invalid fill definition '", stringify!($x), "'")),
        };

        F
    }};
}
