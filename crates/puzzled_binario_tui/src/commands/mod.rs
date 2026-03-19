mod action;

pub use action::*;
use puzzled_tui::{ActionResolver, AppContext, Command};

use crate::AppState;

pub type BinarioCommand = Command<BinarioAction, (), ()>;
pub type BinarioResolver = ActionResolver<BinarioAction, (), (), AppState>;
pub type BinarioContext = AppContext<BinarioAction, (), (), AppState>;
