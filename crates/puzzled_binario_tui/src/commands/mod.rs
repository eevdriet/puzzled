mod action;

pub use action::*;
use puzzled_tui::AppTypes;

use crate::AppState;

pub struct BinarioApp;

impl AppTypes for BinarioApp {
    type Action = BinarioAction;
    type TextObject = ();
    type Motion = ();
    type State = AppState;
}
