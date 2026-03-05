use tokio::sync::mpsc;

use crate::{ActionOutcome, StatefulScreen};

pub struct ActionResolver<A, T> {
    pub(crate) sender: mpsc::UnboundedSender<ActionOutcome<A, T>>,
}

impl<A, T> ActionResolver<A, T> {
    pub(crate) fn new(sender: mpsc::UnboundedSender<ActionOutcome<A, T>>) -> Self {
        Self { sender }
    }

    pub fn next_screen(&self, screen: Box<dyn StatefulScreen<A, T>>) {
        self.sender
            .send(ActionOutcome::NextScreen(screen))
            .expect("Should be able to resolve next screen");
    }

    pub fn prev_screen(&self) {
        self.sender
            .send(ActionOutcome::PreviousScreen)
            .expect("Should be able to resolve previous screen");
    }

    pub fn replace_screen(&self, screen: Box<dyn StatefulScreen<A, T>>) {
        self.sender
            .send(ActionOutcome::ReplaceScreen(screen))
            .expect("Should be able to resolve replace screen");
    }

    pub fn exit(&self) {
        self.sender
            .send(ActionOutcome::Exit)
            .expect("Should be able to resolve exit");
    }
}

impl<A, T> Clone for ActionResolver<A, T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
