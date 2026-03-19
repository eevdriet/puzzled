use tokio::sync::mpsc;

use crate::{Command, CommandOutcome, EventMode, Popup, StatefulScreen};

pub struct ActionResolver<A, T, M, S> {
    pub(crate) sender: mpsc::UnboundedSender<CommandOutcome<A, T, M, S>>,
}

impl<A, T, M, S> ActionResolver<A, T, M, S> {
    pub(crate) fn new(sender: mpsc::UnboundedSender<CommandOutcome<A, T, M, S>>) -> Self {
        Self { sender }
    }

    pub fn next_screen(&self, screen: Box<dyn StatefulScreen<A, T, M, S>>) {
        self.sender
            .send(CommandOutcome::NextScreen(screen))
            .expect("Should be able to resolve next screen");
    }

    pub fn prev_screen(&self) {
        self.sender
            .send(CommandOutcome::PreviousScreen)
            .expect("Should be able to resolve previous screen");
    }

    pub fn open_popup(&self, popup: Box<dyn Popup<A, T, M, S>>) {
        self.sender
            .send(CommandOutcome::OpenPopup(popup))
            .expect("Should be able to resolve next screen");
    }

    pub fn close_popup(&self) {
        self.sender
            .send(CommandOutcome::ClosePopup)
            .expect("Should be able to resolve next screen");
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

    pub fn fire_command(&self, command: Command<A, T, M>) {
        self.sender
            .send(CommandOutcome::Command(command))
            .expect("Should be able to resolve command");
    }
}

impl<A, T, M, S> Clone for ActionResolver<A, T, M, S> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
