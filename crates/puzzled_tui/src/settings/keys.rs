use std::collections::HashMap;

use config::ConfigError;
use serde::Deserialize;

use crate::{
    Action, AppTypes, EventTrie, Load, Motion, Operator, SingleOrMultiple, TextObject, TrieEntry,
    parse_key,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct KeysConfig<A: AppTypes> {
    #[serde(default = "HashMap::default")]
    actions: HashMap<Action<A::Action>, SingleOrMultiple<String>>,

    #[serde(default = "HashMap::default")]
    motions: HashMap<Motion<A::Motion>, SingleOrMultiple<String>>,

    #[serde(default = "HashMap::default")]
    operators: HashMap<Operator, SingleOrMultiple<String>>,

    #[serde(default = "HashMap::default")]
    text_objects: HashMap<TextObject<A::TextObject>, SingleOrMultiple<String>>,
}

impl<A: AppTypes> Load<'_> for KeysConfig<A> {
    const FILE_NAME: &'static str = "keys";
}

impl<A: AppTypes> Default for KeysConfig<A> {
    fn default() -> Self {
        Self {
            actions: HashMap::default(),
            motions: HashMap::default(),
            operators: HashMap::default(),
            text_objects: HashMap::default(),
        }
    }
}

impl<A: AppTypes> EventTrie<A> {
    pub fn load() -> Result<Self, ConfigError> {
        let keys = KeysConfig::<A>::load::<A::Puzzle>()?;

        let mut trie = EventTrie::default();

        let mut insert = |entry: TrieEntry<A::Action, A::TextObject, A::Motion>,
                          key_seq: SingleOrMultiple<String>|
         -> Result<(), String> {
            let key_strs = match key_seq {
                SingleOrMultiple::Single(single) => vec![single],
                SingleOrMultiple::Multiple(keys) => keys,
            };

            for key_str in key_strs {
                let events = parse_key(&key_str)?;
                trie.insert(&events, entry.clone());
            }

            Ok(())
        };

        for (action, key_seq) in keys.actions {
            let entry = TrieEntry::Action(action);
            insert(entry, key_seq).map_err(ConfigError::Message)?;
        }
        for (motion, key_seq) in keys.motions {
            let entry = TrieEntry::Motion(motion);
            insert(entry, key_seq).map_err(ConfigError::Message)?;
        }
        for (op, key_seq) in keys.operators {
            let entry = TrieEntry::Operator(op);
            insert(entry, key_seq).map_err(ConfigError::Message)?;
        }
        for (obj, key_seq) in keys.text_objects {
            let entry = TrieEntry::TextObject(obj);
            insert(entry, key_seq).map_err(ConfigError::Message)?;
        }

        Ok(trie)
    }
}
