use derive_more::{Display, Eq};
use puzzled_core::{Grid, Position};
use puzzled_crossword::{Clue, ClueDirection, ClueId, Crossword};
use puzzled_tui::{Description, GridRenderState, HandleCustomMotion, MotionBehavior};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CrosswordMotion {
    Clue(
        #[serde(skip, default)]
        #[eq(skip)]
        u8,
    ),
}

impl Description<()> for CrosswordMotion {
    fn description(&self, _state: &()) -> Option<String> {
        let desc = match self {
            CrosswordMotion::Clue(_) => "Not inside a clue",
        };

        Some(desc.to_string())
    }
}

pub(crate) struct GridMotionState<'a> {
    pub(crate) puzzle: &'a Crossword,
}

impl<'a, T> HandleCustomMotion<CrosswordMotion, GridRenderState, GridMotionState<'a>, Position>
    for Grid<T>
{
    fn handle_custom_motion(
        &self,
        _count: usize,
        motion: CrosswordMotion,
        render: &mut GridRenderState,
        custom_state: &mut GridMotionState,
    ) -> impl IntoIterator<Item = Position> {
        match motion {
            CrosswordMotion::Clue(num) => {
                let pos = render.cursor;
                let direction = ClueDirection::from(render.direction);
                let id = ClueId { num, direction };

                let clue = match custom_state.puzzle.clues().get(&id) {
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
