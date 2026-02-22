use crate::{Timer, Version};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Metadata {
    /// Author of the puzzle
    pub author: Option<String>,

    /// Copyright of the puzzle
    pub copyright: Option<String>,

    /// Notes on the puzzle
    pub notes: Option<String>,

    /// Title of the puzzle
    pub title: Option<String>,

    /// Version of the puzzle
    pub version: Option<Version>,

    /// Timer that keeps track of playing time
    pub timer: Timer,
}

#[macro_export]
macro_rules! add_metadata {
    ($ty:ty) => {
        impl $ty {
            pub fn author(&self) -> Option<&str> {
                self.meta.author.as_deref()
            }

            pub fn copyright(&self) -> Option<&str> {
                self.meta.copyright.as_deref()
            }

            pub fn notes(&self) -> Option<&str> {
                self.meta.notes.as_deref()
            }

            pub fn title(&self) -> Option<&str> {
                self.meta.title.as_deref()
            }

            pub fn timer(&self) -> $crate::Timer {
                self.meta.timer.clone()
            }

            pub fn timer_mut(&mut self) -> &mut $crate::Timer {
                &mut self.meta.timer
            }

            pub fn version(&self) -> Option<$crate::Version> {
                self.meta.version.clone()
            }
        }
    };
}

#[cfg(feature = "serde")]
mod serde_impl {
    use crate::{Metadata, Timer, Version};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct SerdeMetadata {
        #[serde(skip_serializing_if = "Option::is_none")]
        author: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        copyright: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,

        // Metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        timer: Option<Timer>,

        #[serde(skip_serializing_if = "Option::is_none")]
        version: Option<Version>,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Metadata {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let Metadata {
                author,
                copyright,
                notes,
                title,
                version,
                timer,
            } = self.clone();

            SerdeMetadata {
                author,
                copyright,
                notes,
                title,
                version,
                timer: Some(timer),
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Metadata {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let SerdeMetadata {
                author,
                copyright,
                notes,
                title,
                timer,
                version,
            } = SerdeMetadata::deserialize(deserializer)?;

            Ok(Metadata {
                author,
                copyright,
                notes,
                title,
                version,
                timer: timer.unwrap_or_default(),
            })
        }
    }
}
