use std::{fmt::Debug, hash::Hash};

use puzzled_core::Puzzle;
use serde::de::DeserializeOwned;

use crate::{ActionBehavior, Describe, MotionBehavior, TextObjectBehavior};

pub trait AppTypeTraits:
    Clone
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Send
    + Debug
    + Sized
    + Describe
    + Hash
    + DeserializeOwned
{
}

impl<
    T: Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Send
        + Debug
        + Sized
        + Describe
        + Hash
        + DeserializeOwned,
> AppTypeTraits for T
{
}

pub trait AppTypes {
    // Puzzle
    type Puzzle: Puzzle;

    // Commands
    type Action: ActionBehavior;
    type TextObject: TextObjectBehavior;
    type Motion: MotionBehavior;

    // App
    type State;
}
