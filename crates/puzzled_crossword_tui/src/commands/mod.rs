mod action;
mod motion;
mod text_obj;

pub use action::*;
pub use motion::*;
use puzzled_tui::{AppContext, AppResolver, AppTypes, Command, KeyMap};
pub use text_obj::*;

use crate::AppState;

pub struct CrosswordApp;

impl AppTypes for CrosswordApp {
    type Action = CrosswordAction;
    type TextObject = CrosswordTextObject;
    type Motion = CrosswordMotion;
    type State = AppState;
}
