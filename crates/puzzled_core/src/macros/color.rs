#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! color {
    // rgba(r, g, b, a)
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        $crate::Color::rgba($r, $g, $b, $a)
    };

    // rgb(r, g, b)
    ($r:expr, $g:expr, $b:expr) => {
        $crate::Color::rgb($r, $g, $b)
    };

    // hex string literal
    ($hex:literal) => {{
        match $crate::Color::hex($hex) {
            Ok(color) => color,
            Err(err) => panic!("{}", err),
        }
    }};

    // everything else → compile error
    ($($invalid:tt)*) => {
        compile_error!(concat!(
            "Invalid color '",
            stringify!($($invalid)*),
            "', use one of:\n\
             color!(r, g, b)\n\
             color!(r, g, b, a)\n\
             color!(\"#RRGGBB\" | \"#RGB\" | \"#RRGGBBAA\" | \"#RGBA\")"
        ));
    };
}
