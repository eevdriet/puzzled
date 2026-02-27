#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! rule {
    ($line:ident $num:literal : [$($count:literal $fill:tt),+] $len:literal) => {
        let line = $crate::line!($line $num);
        let runs = vec![$( $crate::run!($count $fill)),+];

        $crate::Rule::new(runs, $len)
    };

    ($($invalid:tt)*) => {{
        $crate::macro_error($($invalid)*, "rule!")
    }};
}
