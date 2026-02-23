//! Defines all functionality for formatting the various [*.puz data][PUZ google spec]
//!
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki
mod error;
mod string;

pub use error::*;
pub use string::Error as StringError;
