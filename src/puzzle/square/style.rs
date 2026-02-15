use bitflags::bitflags;

use crate::Cell;

bitflags! {
    /// Style that changes the way a square is displayed
    ///
    /// The style is represented as [bit flags](bitflags) such that multiple styles can simultaneously be set.
    /// Currently, 4 styles are defined.
    /// The definitions derive from the **GEXT data section** of the [*.puz file documentation](wiki).
    ///
    /// ```rust
    /// use puzzled::CellStyle;
    ///
    /// let style = CellStyle::REVEALED | CellStyle::CIRCLED;
    /// ```
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct CellStyle: u16 {
        /// Square previously contained an incorrect guess
        const PREVIOUSLY_INCORRECT  = 0x10;

        /// Square currently contains an incorrect guess
        const INCORRECT             = 0x20;

        /// Square is manually revealed by the user to show its solution
        const REVEALED              = 0x40;

        /// Square is circled.
        /// This is commonly used for solutions that reveal an inner solution.
        /// E.g. `SHOC(K)(I)(N)(G)(P)(I)(N)K` reveals `KINGPIN`
        const CIRCLED               = 0x80;
    }
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
/// fn is_revealed(self) -> T {
///     !(self.style & CellStyle::REVEALED).is_empty()!
/// }
/// ```
macro_rules! check_style {
    ($variant:expr, $style_fn:ident()) => {
        #[doc = concat!("Checks whether [`", stringify!($variant), "`]) is set.")]
        pub fn $style_fn(&self) -> bool {
            !(self.style & $variant).is_empty()
        }
    };
}

impl Cell {
    check_style!(CellStyle::REVEALED, is_revealed());
    check_style!(CellStyle::CIRCLED, is_circled());
    check_style!(CellStyle::INCORRECT, is_incorrect());
    check_style!(CellStyle::PREVIOUSLY_INCORRECT, was_incorrect());
}
