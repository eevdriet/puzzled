use std::collections::HashMap;

use serde::Deserialize;

use crate::{Action, EventTrie, parse_key};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum RawKeySeq {
    Single(String),
    Multiple(Vec<String>),
}

pub(crate) type RawActionKeys<A> = HashMap<Action<A>, RawKeySeq>;

pub(crate) fn parse_action_events<A>(action_keys: RawActionKeys<A>) -> Result<EventTrie<A>, String>
where
    A: Clone,
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
