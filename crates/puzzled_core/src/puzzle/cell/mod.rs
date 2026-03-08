mod style;

use std::fmt::{self, Debug};

pub use style::CellStyle;

use crate::Value;

pub const MISSING_ENTRY_CHAR: char = '-';

pub struct Cell<T> {
    pub solution: Option<T>,
    pub style: CellStyle,
}

impl<T> Cell<T> {
    pub fn new(value: Option<T>) -> Self {
        let style = CellStyle::default();
        Self::new_with_style(value, style)
    }

    pub fn new_with_style(solution: Option<T>, style: CellStyle) -> Self {
        Self { solution, style }
    }

    pub fn default_with_style(style: CellStyle) -> Self {
        Self {
            solution: None,
            style,
        }
    }
}

impl<T> fmt::Debug for Cell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cell = f.debug_struct("Cell");
        cell.field("solution", &self.solution);
        cell.field("style", &self.style);

        cell.finish()
    }
}

impl<T> fmt::Display for Cell<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.solution {
                None => MISSING_ENTRY_CHAR.to_string(),
                Some(ref sol) => sol.to_string(),
            },
            self.style
        )
    }
}

impl<T> Value<T> for Cell<T> {
    fn value(&self) -> Option<&T> {
        self.solution.as_ref()
    }

    fn value_mut(&mut self) -> Option<&mut T> {
        self.solution.as_mut()
    }
}

impl<T> Default for Cell<T> {
    fn default() -> Self {
        Self {
            solution: None,
            style: CellStyle::empty(),
        }
    }
}

impl<T> PartialEq for Cell<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.solution == other.solution
    }
}

impl<T> Eq for Cell<T> where T: Eq {}

impl<T> Clone for Cell<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            solution: self.solution.clone(),
            style: self.style,
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize, ser::SerializeStruct};

    use crate::{Cell, CellStyle};

    #[derive(Deserialize)]
    struct SerdeCell<T> {
        value: T,
        style: CellStyle,
    }

    impl<T> Serialize for Cell<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut cell = serializer.serialize_struct("Cell", 2)?;
            cell.serialize_field("value", &self.solution)?;
            cell.serialize_field("style", &self.style)?;

            cell.end()
        }
    }

    impl<'de, T> Deserialize<'de> for Cell<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let cell = SerdeCell::deserialize(deserializer)?;
            let cell = Cell {
                solution: cell.value,
                style: cell.style,
            };

            Ok(cell)
        }
    }
}
