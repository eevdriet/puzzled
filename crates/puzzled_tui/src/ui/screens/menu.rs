use ratatui::widgets::ListState;

use crate::HandleAction;

pub struct Menu<A, T> {
    title: String,
    options: Vec<Box<dyn HandleAction<A, T, State = T>>>,

    state: ListState,
}
