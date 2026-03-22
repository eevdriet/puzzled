use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    Action, ActionBehavior, AppTypes, EventTrie, Motion, MotionBehavior, Operator, TextObject,
    TextObjectBehavior, TrieEntry, parse_key,
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum RawKeySeq {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename = "kebab-case")]
pub(crate) struct RawKeys<A, T, M>
where
    Action<A>: ActionBehavior,
    TextObject<T>: TextObjectBehavior,
    Motion<M>: MotionBehavior,
{
    #[serde(default = "HashMap::default")]
    actions: HashMap<Action<A>, RawKeySeq>,

    #[serde(default = "HashMap::default")]
    motions: HashMap<Motion<M>, RawKeySeq>,

    #[serde(default = "HashMap::default")]
    operators: HashMap<Operator, RawKeySeq>,

    #[serde(default = "HashMap::default")]
    text_objects: HashMap<TextObject<T>, RawKeySeq>,
}

pub(crate) fn insert_action_keys<A: AppTypes>(
    keys: RawKeys<A::Action, A::TextObject, A::Motion>,
    trie: &mut EventTrie<A>,
) -> Result<(), String> {
    let mut insert = |entry: TrieEntry<A::Action, A::TextObject, A::Motion>,
                      key_seq: RawKeySeq|
     -> Result<(), String> {
        let key_strs = match key_seq {
            RawKeySeq::Single(single) => vec![single],
            RawKeySeq::Multiple(keys) => keys,
        };

        for key_str in key_strs {
            let events = parse_key(&key_str)?;
            trie.insert(&events, entry.clone());
        }

        Ok(())
    };

    for (action, key_seq) in keys.actions {
        let entry = TrieEntry::Action(action);
        insert(entry, key_seq)?;
    }
    for (motion, key_seq) in keys.motions {
        let entry = TrieEntry::Motion(motion);
        insert(entry, key_seq)?;
    }
    for (op, key_seq) in keys.operators {
        let entry = TrieEntry::Operator(op);
        insert(entry, key_seq)?;
    }
    for (obj, key_seq) in keys.text_objects {
        let entry = TrieEntry::TextObject(obj);
        insert(entry, key_seq)?;
    }

    Ok(())
}
