#![cfg_attr(docsrs, feature(doc_cfg))]
/// Macro for constructing a [cell](crate::Cell) inline
///
/// Squares can be constructed a 2 different ways
/// 1. A [`char`] constructs a [letter](crate::Solution::Letter) cell
/// 2. A [`str`] constructs a [rebus](crate::Solution::Rebus) cell
///
/// Note that a [`square!`](crate::square) can be constructed with the same syntax
/// TODO: rewrite examples
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! cell {
// Rebus or Letter with optional entry and optional styles
    // Solution + styles
    ($sol:tt $($style:tt)+) => {{
        let style = $crate::cell_style!($($style)+);

        // Create the cell from solution and style
        $crate::Cell::new_with_style($sol, style)
    }};


    // Solution
    ($sol:expr) => {{
        $crate::Cell::new($sol)
    }};

    ($($invalid:tt)*) => {{
        $crate::__error($($invalid)*, "cell!")
    }};
}

#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
/// Cell style
macro_rules! cell_style {
    () => {
        $crate::CellStyle::default()
    };

    (~ $($rest:tt)*) => {
        $crate::CellStyle::PREVIOUSLY_INCORRECT
        | $crate::cell_style!($($rest)*)
    };

    (* $($rest:tt)*) => {
        $crate::CellStyle::REVEALED
        | $crate::cell_style!($($rest)*)
    };

    (@ $($rest:tt)*) => {
        $crate::CellStyle::CIRCLED
        | $crate::cell_style!($($rest)*)
    };

    ($invalid:tt $($rest:tt)*) => {
        compile_error!(
            concat!(
                "Unknown style suffix: '",
                stringify!($invalid),
                "' (only ~, * and @ allowed)"
            )
        );
    };
}

/// Generates a method to check whether a style is set on a [`FillSquare`]
///
/// # Examples
///
/// ```rust,ignore
/// check_style!(CellStyle::REVEALED, is_revealed());
///
/// // generates
///
/// #[doc = "Checks whether [`CellStyle::REVEALED`] is set."]
/// fn is_revealed(self) -> bool {
///     self.style.contains(CellStyle::REVEALED)
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! check_style {
    ($variant:expr, $field:ident, $style_fn:ident()) => {
        #[doc = concat!("Checks whether [`", stringify!($variant), "`]) is set.")]
        pub fn $style_fn(&self) -> bool {
            self.$field.contains($variant)
        }
    };
}
