use derive_more::{Display, Eq};
use puzzled_core::Grid;
use puzzled_crossword::{Clue, ClueDirection, ClueId};
use puzzled_tui::{CustomMotionRange, Describe, MotionBehavior};
use serde::Deserialize;

use crate::PuzzleScreenState;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CrosswordMotion {
    Clue(
        #[serde(skip, default)]
        #[eq(skip)]
        u8,
    ),
}

impl Describe for CrosswordMotion {
    fn describe(&self) -> Option<String> {
        None
    }
}

impl<T> CustomMotionRange<CrosswordMotion, PuzzleScreenState> for Grid<T> {
    fn custom_motion_range(
        &self,
        _start: Self::Position,
        _count: usize,
        motion: &CrosswordMotion,
        state: &PuzzleScreenState,
    ) -> impl IntoIterator<Item = Self::Position> {
        match motion {
            CrosswordMotion::Clue(num) => {
                let pos = state.render.cursor;
                let direction = ClueDirection::from(state.render.direction);
                let id = ClueId {
                    num: *num,
                    direction,
                };

                let clue = match state.puzzle.clues().get(&id) {
                    Some(c) => c.clone(),
                    None => Clue::new(0, ClueDirection::Across, "", pos, 0),
                };

                clue.positions().collect::<Vec<_>>()
            }
        }
    }
}

impl MotionBehavior for CrosswordMotion {
    fn variants() -> Vec<Self> {
        let clue = 0;

        vec![Self::Clue(clue)]
    }
}
