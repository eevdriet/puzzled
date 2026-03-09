use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Display, Hash)]
pub enum EventMode {
    Normal,
    Insert,
    Replace,
}
