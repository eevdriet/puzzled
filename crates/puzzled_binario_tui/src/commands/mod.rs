mod action;

pub use action::*;
use puzzled_tui::{AppContext, AppResolver, Command};

use crate::AppState;

pub type BinarioCommand = Command<BinarioAction, (), ()>;
pub type BinarioResolver = AppResolver<BinarioAction, (), (), AppState>;
pub type BinarioContext = AppContext<BinarioAction, (), (), AppState>;
