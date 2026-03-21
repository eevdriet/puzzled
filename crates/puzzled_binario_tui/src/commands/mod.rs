mod action;

pub use action::*;
use puzzled_binario::Binario;
use puzzled_tui::AppTypes;

use crate::AppState;

pub struct BinarioApp;

impl AppTypes for BinarioApp {
    type Puzzle = Binario;
    type Action = BinarioAction;
    type TextObject = ();
    type Motion = ();
    type State = AppState;
}
