mod keys;
mod options;
mod theme;

pub use options::*;
pub use theme::*;

use config::{Config, ConfigError, File};

use puzzled_core::Puzzle;
use puzzled_io::{config_dir, puzzle_config_dir};
use serde::Deserialize;

use crate::{AppTypes, EventTrie};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum SingleOrMultiple<T> {
    Single(T),
    Multiple(Vec<T>),
}

#[derive(Debug)]
pub struct Settings<A: AppTypes> {
    pub keys: EventTrie<A>,
    pub options: Options,
    pub theme: Theme,
}

impl<A: AppTypes> Settings<A> {
    pub fn load() -> Result<Self, ConfigError> {
        let keys = EventTrie::<A>::load()?;

        let options = Options::load::<A::Puzzle>()?;
        let theme = Theme::from_palette(Palette::SOLARIZED);

        Ok(Self {
            keys,
            options,
            theme,
        })
    }
}

pub trait Load<'de>: Deserialize<'de> {
    const FILE_NAME: &'static str;

    fn load<P: Puzzle>() -> Result<Self, ConfigError> {
        let global_path = config_dir()
            .map_err(|err| ConfigError::Message(err.to_string()))?
            .join(Self::FILE_NAME)
            .with_extension("toml")
            .to_str()
            .expect("Valid path")
            .to_string();

        let puzzle_path = puzzle_config_dir::<P>()
            .map_err(|err| ConfigError::Message(err.to_string()))?
            .join(Self::FILE_NAME)
            .with_extension("toml")
            .to_str()
            .to_owned()
            .expect("Valid path")
            .to_string();

        let settings = Config::builder()
            .add_source(File::with_name(&global_path).required(false))
            .add_source(File::with_name(&puzzle_path).required(false))
            .build()?;

        settings.try_deserialize()
    }
}
