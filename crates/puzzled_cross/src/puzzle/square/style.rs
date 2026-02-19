use crate::Cell;
use std::ops;

/// Style that changes the way a [cell](Cell) is displayed
///
/// The style is represented as *bit flags* such that multiple styles can simultaneously be set.
/// Currently, the 4 styles that are defined are
/// - [`PREVIOUSLY_INCORRECT`](CellStyle::PREVIOUSLY_INCORRECT) (`0x10`) for cells that previously contained an [incorrect](Cell::is_correct) guess
/// - [`INCORRECT`](CellStyle::INCORRECT) (`0x20`) for cells that currently contain an [incorrect](Cell::is_correct) guess
/// - [`REVEALED`](CellStyle::REVEALED) (`0x40`) for cells that are manually [revealed](Cell::reveal) by the user to show their solution
/// - [`CIRCLED`](CellStyle::CIRCLED) (`0x80`) for cells that are circled
///
/// The definitions derive from the **GEXT data section** of the [*.puz spefication](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki).
///
/// ```rust
/// use puzzled_crossword::{Cell, CellStyle, Solution};
///
/// let style = CellStyle::INCORRECT | CellStyle::CIRCLED;
/// let mut cell = Cell::new_styled(Solution::Letter('A'), style);
/// assert!(cell.is_incorrect());
/// assert!(cell.is_circled());
///
/// assert!(!cell.is_revealed());
/// cell.reveal();
/// assert!(cell.is_revealed());
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct CellStyle(u8);

impl CellStyle {
    /// [Cell] does not contains any style
    pub const EMPTY: CellStyle = CellStyle(0x0);

    /// [Cell] previously contained an [incorrect](Cell::is_correct) guess
    pub const PREVIOUSLY_INCORRECT: CellStyle = CellStyle(0x10);

    /// [Cell] currently contains an [incorrect](Cell::is_correct) guess
    pub const INCORRECT: CellStyle = CellStyle(0x20);

    /// [Cell] is manually [revealed](Cell::reveal) by the user to show its solution
    pub const REVEALED: CellStyle = CellStyle(0x40);

    /// [Cell] is circled.
    ///
    /// This is commonly used for solutions that reveal an inner solution.
    /// E.g. `SHOC(K)(I)(N)(G)(P)(I)(N)K` reveals `KINGPIN`
    pub const CIRCLED: CellStyle = CellStyle(0x80);

    /// Verify whether the cell style is [empty](CellStyle::EMPTY)
    pub fn is_empty(&self) -> bool {
        *self == CellStyle::EMPTY
    }

    /// Retrieve the style bits
    pub fn bits(&self) -> u8 {
        self.0
    }

    /// Try to construct a valid style from the underlying data type
    pub fn from_mask(mut mask: u8) -> Option<Self> {
        let mut result = CellStyle::EMPTY;

        // Try to apply each of the style bits
        let styles = [
            CellStyle::PREVIOUSLY_INCORRECT,
            CellStyle::INCORRECT,
            CellStyle::REVEALED,
            CellStyle::CIRCLED,
        ];

        for style in styles {
            // Add style to the result if style bit is set
            if mask & 1 != 0 {
                result |= style;
            }

            mask >>= 1;
        }

        // Only construct style if no other bits are set
        match mask {
            0 => Some(result),
            _ => None,
        }
    }
}

impl Default for CellStyle {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl ops::BitOr<CellStyle> for CellStyle {
    type Output = CellStyle;

    fn bitor(self, rhs: CellStyle) -> CellStyle {
        CellStyle(self.0 | rhs.0)
    }
}

impl ops::BitAnd<CellStyle> for CellStyle {
    type Output = CellStyle;

    fn bitand(self, rhs: CellStyle) -> CellStyle {
        CellStyle(self.0 & rhs.0)
    }
}

impl ops::BitOrAssign<CellStyle> for CellStyle {
    fn bitor_assign(&mut self, rhs: CellStyle) {
        *self = *self | rhs;
    }
}

impl ops::BitAndAssign<CellStyle> for CellStyle {
    fn bitand_assign(&mut self, rhs: CellStyle) {
        *self = *self & rhs;
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
///     self.style & CellStyle::REVEALED != CellStyle::EMPTY
/// }
/// ```
macro_rules! check_style {
    ($variant:expr, $style_fn:ident()) => {
        #[doc = concat!("Checks whether [`", stringify!($variant), "`]) is set.")]
        pub fn $style_fn(&self) -> bool {
            self.style() & $variant != CellStyle::EMPTY
        }
    };
}

impl Cell {
    check_style!(CellStyle::REVEALED, is_revealed());
    check_style!(CellStyle::CIRCLED, is_circled());
    check_style!(CellStyle::INCORRECT, is_incorrect());
    check_style!(CellStyle::PREVIOUSLY_INCORRECT, was_incorrect());
}
