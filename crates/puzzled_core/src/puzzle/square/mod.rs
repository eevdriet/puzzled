use std::fmt::{self, Display};

use derive_more::{Deref, DerefMut};

use crate::Word;

pub const NON_PLAYABLE_CHAR: char = '.';

#[derive(Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Square<T>(pub Option<T>);

impl<T> Square<T> {
    pub fn new(value: T) -> Self {
        Self(Some(value))
    }

    pub fn new_empty() -> Self {
        Self(None)
    }

    pub fn map_ref<U, F>(&self, f: F) -> Square<U>
    where
        F: FnOnce(&T) -> Option<U>,
    {
        let mapped = match self.0 {
            None => None,
            Some(ref value) => f(value),
        };

        Square(mapped)
    }

    pub fn symbol(&self) -> String
    where
        T: Display,
    {
        match self.as_ref() {
            Some(val) => val.to_string(),
            _ => "".to_string(),
        }
    }
}

impl<T> Word for Square<T>
where
    T: Word,
{
    fn is_word(&self) -> bool {
        self.as_ref().is_some_and(|square| square.is_word())
    }
}

impl<T> fmt::Display for Square<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.as_ref() {
                None => NON_PLAYABLE_CHAR.to_string(),
                Some(sol) => sol.to_string(),
            }
        )
    }
}

impl<T> Default for Square<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Clone for Square<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Square;

    impl<T> Serialize for Square<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for Square<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let square = Option::<T>::deserialize(deserializer)?;
            Ok(Square(square))
        }
    }
}
