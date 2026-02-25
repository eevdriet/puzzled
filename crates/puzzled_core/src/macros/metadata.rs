#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! metadata {
    ( $( $key:ident : $value:expr),* $(,)? ) => {{
        let mut metadata = $crate::Metadata::default();

        $(
            $key: $crate::metadata!(@transform metadata, $key, $value),
        )*

        metadata
    }};

     // String parsing
    (@transform $meta:ident, author, $value:literal) => {
        $meta = $meta.with_author($value.into())
    };

    (@transform $meta:ident, copyright, $value:literal) => {
        $meta = $meta.with_copyright($value.into())
    };

    (@transform $meta:ident, notes, $value:literal) => {
        $meta = $meta.with_notes($value.into())
    };

    (@transform $meta:ident, title, $value:literal) => {
        $meta = $meta.with_title($value.into())
    };

    // Version parsing
    (@transform $meta:ident, version, $value:literal) => {
        match $crate::Version::new($value.as_bytes()) {
            Ok(v) => $meta = $meta.with_version(v),
            Err(_) => panic!("Invalid version string"),
        }
    };

    // Timer parsing
    (@transform $meta:ident, timer, $value:literal) => {
        match $crate::Timer::from_str($value) {
            Ok(t) => $meta = $meta.with_timer(t),
            Err(_) => panic!("Invalid timer string"),
        }
    };

    // Unknown key
    (@transform $key:ident, $value:literal) => {
        compile_error!(concat!("Unknown metadata: ", stringify!($key)))
    };
}
