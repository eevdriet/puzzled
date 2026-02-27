#[doc(hidden)]
#[macro_export]
macro_rules! line {
    (R $num:literal) => {
        $crate::Line::Row($num)
    };

    (C $num:literal) => {
        $crate::Line::Row($num)
    };

    ($($invalid:tt)*) => {{
        $crate::macro_error($($invalid)*, "line!")
    }};
}
