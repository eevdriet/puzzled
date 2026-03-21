use std::collections::HashMap;

use serde::Deserialize;

use crate::{AppTypes, EventTrie, TrieEntry, parse_key};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum RawKeySeq {
    Single(String),
    Multiple(Vec<String>),
}

pub(crate) type RawActionKeys<A, T, M> = HashMap<TrieEntry<A, T, M>, RawKeySeq>;

pub(crate) fn insert_action_keys<A: AppTypes>(
    action_keys: RawActionKeys<A::Action, A::TextObject, A::Motion>,
    trie: &mut EventTrie<A>,
) -> Result<(), String> {
    for (action, key_seq) in action_keys {
        let key_strs = match key_seq {
            RawKeySeq::Single(single) => vec![single],
            RawKeySeq::Multiple(keys) => keys,
        };

        for key_str in key_strs {
            let events = parse_key(&key_str)?;
            trie.insert(&events, action.clone());
        }
    }

    Ok(())
}
