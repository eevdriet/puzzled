use puzzled_nonogram::Nonogram;
use puzzled_tui::AppTypes;

pub struct NonogramApp;

impl AppTypes for NonogramApp {
    type Puzzle = Nonogram;
    type Action = ();
    type TextObject = ();
    type Motion = ();
    type State = ();
}
