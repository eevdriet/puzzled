mod timer;
mod version;

pub use timer::{Error as TimerError, Timer, TimerState};
pub use version::{Error as VersionError, Version};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Metadata {
    /// Author of the puzzle
    author: Option<String>,

    /// Copyright of the puzzle
    copyright: Option<String>,

    /// Notes on the puzzle
    notes: Option<String>,

    /// Title of the puzzle
    title: Option<String>,

    /// Version of the puzzle
    version: Option<Version>,
}

impl Metadata {
    /// Author of the puzzle
    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    /// Copyright information of the puzzle
    pub fn copyright(&self) -> Option<&str> {
        self.copyright.as_deref()
    }

    /// Notes on the puzzle
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Title on the puzzle
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Version of the puzzle
    pub fn version(&self) -> Option<Version> {
        self.version
    }

    /// Define the author of the puzzle
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Define copyright information of the puzzle
    pub fn with_copyright(mut self, copyright: String) -> Self {
        self.copyright = Some(copyright);
        self
    }

    /// Define notes on the puzzle
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }

    /// Define the author of the puzzle
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Define the version of the puzzle
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use crate::{Metadata, Version};
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
            } = self.clone();

            SerdeMetadata {
                author,
                copyright,
                notes,
                title,
                version,
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
                version,
            } = SerdeMetadata::deserialize(deserializer)?;

            Ok(Metadata {
                author,
                copyright,
                notes,
                title,
                version,
            })
        }
    }
}
