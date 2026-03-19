mod action;
mod motion;
mod text_obj;

pub use action::*;
pub use motion::*;
use puzzled_tui::{ActionResolver, AppContext, Command, Keys};
pub use text_obj::*;

use crate::AppState;

pub type CrosswordCommand = Command<CrosswordAction, CrosswordTextObject, CrosswordMotion>;
pub type CrosswordResolver =
    ActionResolver<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>;
pub type CrosswordContext =
    AppContext<CrosswordAction, CrosswordTextObject, CrosswordMotion, AppState>;
pub type CrosswordKeys = Keys<CrosswordAction, CrosswordTextObject, CrosswordMotion>;
