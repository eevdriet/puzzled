use tokio::sync::mpsc;

use crate::{AppCommand, AppTypes, CommandOutcome, EventMode, Screen};

pub struct AppResolver<A: AppTypes> {
    pub(crate) sender: mpsc::UnboundedSender<CommandOutcome<A>>,
}

impl<A: AppTypes> AppResolver<A> {
    pub(crate) fn new(sender: mpsc::UnboundedSender<CommandOutcome<A>>) -> Self {
        Self { sender }
    }

    pub fn next_screen(&self, screen: Box<dyn Screen<A>>) {
        self.sender
            .send(CommandOutcome::NextScreen(screen))
            .expect("Should be able to resolve next screen");
    }

    pub fn prev_screen(&self) {
        self.sender
            .send(CommandOutcome::PreviousScreen)
            .expect("Should be able to resolve previous screen");
    }

    pub fn open_popup(&self) {
        self.sender
            .send(CommandOutcome::OpenPopup)
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

    pub fn fire_command(&self, command: AppCommand<A>) {
        self.sender
            .send(CommandOutcome::Command(command))
            .expect("Should be able to resolve command");
    }
}

impl<A: AppTypes> Clone for AppResolver<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
