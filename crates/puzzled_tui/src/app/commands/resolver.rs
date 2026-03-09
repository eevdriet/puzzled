use tokio::sync::mpsc;

use crate::{Command, CommandOutcome, StatefulScreen};

pub struct ActionResolver<A, T> {
    pub(crate) sender: mpsc::UnboundedSender<CommandOutcome<A, T>>,
}

impl<A, T> ActionResolver<A, T> {
    pub(crate) fn new(sender: mpsc::UnboundedSender<CommandOutcome<A, T>>) -> Self {
        Self { sender }
    }

    pub fn next_screen(&self, screen: Box<dyn StatefulScreen<A, T>>) {
        self.sender
            .send(CommandOutcome::NextScreen(screen))
            .expect("Should be able to resolve next screen");
    }

    pub fn prev_screen(&self) {
        self.sender
            .send(CommandOutcome::PreviousScreen)
            .expect("Should be able to resolve previous screen");
    }

    pub fn replace_screen(&self, screen: Box<dyn StatefulScreen<A, T>>) {
        self.sender
            .send(CommandOutcome::ReplaceScreen(screen))
            .expect("Should be able to resolve replace screen");
    }

    pub fn quit(&self) {
        self.sender
            .send(CommandOutcome::Quit)
            .expect("Should be able to resolve exit");
    }

    pub fn fire_command(&self, command: Command<A>) {
        self.sender
            .send(CommandOutcome::Command(command))
            .expect("Should be able to resolve command");
    }
}

impl<A, T> Clone for ActionResolver<A, T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
