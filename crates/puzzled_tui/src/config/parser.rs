use std::collections::HashMap;

use serde::Deserialize;

use crate::{EventTrie, TrieEntry, parse_key};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum RawKeySeq {
    Single(String),
    Multiple(Vec<String>),
}

pub(crate) type RawActionKeys<M, A> = HashMap<TrieEntry<M, A>, RawKeySeq>;

pub(crate) fn parse_action_events<M, A>(
    action_keys: RawActionKeys<M, A>,
) -> Result<EventTrie<M, A>, String>
where
    A: Clone,
    M: Clone,
{
    let mut trie = EventTrie::default();

    for (action, key_seq) in action_keys {
        let key_strs = match key_seq {
            RawKeySeq::Single(single) => vec![single],
            RawKeySeq::Multiple(keys) => keys,
        };

        for key_str in key_strs {
            let events = parse_key(&key_str, &action)?;
            trie.insert(&events, action.clone());
        }
    }

    Ok(trie)
}
