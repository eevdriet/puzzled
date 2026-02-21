use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    major: u8,
    minor: u8,
}

impl Version {
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        let err = "Version should be written as `<major>.<minor>` where <major>, <minor> are u8"
            .to_string();

        // Optionally strip the trailing \0
        let version = bytes.strip_suffix(&[0]).unwrap_or(bytes);

        // Version should be 3 components (<major>.<minor>)
        if version.len() != 3 {
            return Err(err);
        }

        // Components should be correct
        let mut bytes = version.iter();
        let (&major, &dot, &minor) = (
            bytes.next().expect("checked version length"),
            bytes.next().expect("checked version length"),
            bytes.next().expect("checked version length"),
        );

        if !(major.is_ascii_digit() && dot == b'.' && minor.is_ascii_digit()) {
            return Err(err);
        }

        Ok(Self { major, minor })
    }

    pub fn new_unchecked(bytes: &[u8]) -> Self {
        Self {
            major: bytes[0],
            minor: bytes[2],
        }
    }

    pub fn as_bytes(&self) -> [u8; 4] {
        [self.major, b'.', self.minor, b'\0']
    }
}

impl Default for Version {
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Version;

    #[derive(Serialize, Deserialize)]
    pub struct SerdeVersion {
        major: u8,
        minor: u8,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            SerdeVersion {
                major: self.major,
                minor: self.minor,
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let SerdeVersion { major, minor } = SerdeVersion::deserialize(deserializer)?;
            let version = Version { major, minor };

            Ok(version)
        }
    }
}
