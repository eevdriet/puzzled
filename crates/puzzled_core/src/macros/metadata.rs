#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! metadata {
    ( $( $key:ident : $value:expr),* $(,)? ) => {
        $crate::Metadata {
            $(
                $key: metadata!(@transform $key, $value),
            )*
            ..Default::default()
        }
    };

     // String parsing
    (@transform author, $value:literal) => {
        Some($value.into())
    };

    (@transform copyright, $value:literal) => {
        Some($value.into())
    };

    (@transform notes, $value:literal) => {
        Some($value.into())
    };

    (@transform title, $value:literal) => {
        Some($value.into())
    };

    // Version parsing
    (@transform version, $value:literal) => {
        match $crate::Version::new($value.as_bytes()) {
            Ok(v) => Some(v),
            Err(_) => panic!("Invalid version string"),
        }
    };

    // Timer parsing
    (@transform timer, $value:literal) => {
        match $crate::Timer::from_str($value) {
            Ok(t) => t,
            Err(_) => panic!("Invalid timer string"),
        }
    };

    // Unknown key
    (@transform $key:ident, $value:literal) => {
        compile_error!(concat!("Unknown metadata: ", stringify!($key)))
    };
}
