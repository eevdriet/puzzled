mod action;
mod motion;
mod text_obj;

pub use action::*;
pub use motion::*;
use puzzled_crossword::Crossword;
use puzzled_tui::AppTypes;
pub use text_obj::*;

use crate::AppState;

pub struct CrosswordApp;

impl AppTypes for CrosswordApp {
    type Puzzle = Crossword;
    type Action = CrosswordAction;
    type TextObject = CrosswordTextObject;
    type Motion = CrosswordMotion;
    type State = AppState;
}
