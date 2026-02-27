#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! run {
    ($count:literal $fill:tt) => {
        const F: $crate::Fill = match $crate::Fill::decode_str($crate::smart_stringify!($fill)) {
            Ok(f) => f,
            Err(_) => panic!(concat!("Invalid fill literal '", stringify!($x), "'")),
        };

        Run::new(F, $count)
    };

    ($($invalid:tt)*) => {{
        $crate::macro_error($($invalid)*, "run!")
    }};
}
