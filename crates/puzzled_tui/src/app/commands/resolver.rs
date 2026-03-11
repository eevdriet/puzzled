use tokio::sync::mpsc;

use crate::{Command, CommandOutcome, EventMode, StatefulScreen};

pub struct ActionResolver<M, A, T> {
    pub(crate) sender: mpsc::UnboundedSender<CommandOutcome<M, A, T>>,
}

impl<M, A, T> ActionResolver<M, A, T> {
    pub(crate) fn new(sender: mpsc::UnboundedSender<CommandOutcome<M, A, T>>) -> Self {
        Self { sender }
    }

    pub fn next_screen(&self, screen: Box<dyn StatefulScreen<M, A, T>>) {
        self.sender
            .send(CommandOutcome::NextScreen(screen))
            .expect("Should be able to resolve next screen");
    }

    pub fn prev_screen(&self) {
        self.sender
            .send(CommandOutcome::PreviousScreen)
            .expect("Should be able to resolve previous screen");
    }

    pub fn quit(&self) {
        self.sender
            .send(CommandOutcome::Quit)
            .expect("Should be able to resolve exit");
    }

    pub fn set_mode(&self, mode: EventMode) {
        self.sender
            .send(CommandOutcome::Mode(mode))
            .expect("Should be able to resolve command");
    }

    pub fn fire_command(&self, command: Command<M, A>) {
        self.sender
            .send(CommandOutcome::Command(command))
            .expect("Should be able to resolve command");
    }
}

impl<M, A, T> Clone for ActionResolver<M, A, T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
