mod action;
mod motion;
mod text_obj;

pub use action::*;
pub use motion::*;
use puzzled_tui::{ActionResolver, AppContext, Command, KeyMap};
pub use text_obj::*;

use crate::AppState;

pub type CrosswordCommand = Command<CrosswordAction, CrosswordTextObject, CrosswordMotion>;
pub type CrosswordResolver =
    ActionResolver<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>;
pub type CrosswordContext =
    AppContext<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>;
pub type CrosswordKeys = KeyMap<CrosswordAction, CrosswordTextObject, CrosswordMotion>;
