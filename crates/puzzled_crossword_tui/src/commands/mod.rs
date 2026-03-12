mod action;
mod motion;
mod text_obj;

pub use action::*;
pub use motion::*;
use puzzled_tui::{ActionResolver, Command};
pub use text_obj::*;

use crate::AppState;

pub type CrosswordCommand = Command<CrosswordAction, CrosswordTextObject, CrosswordMotion>;
pub type CrosswordResolver =
    ActionResolver<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>;
