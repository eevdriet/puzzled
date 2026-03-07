use crate::UndoCommand;

pub struct CommandHistory<T> {
    undos: Vec<Box<dyn UndoCommand<T>>>,
    redos: Vec<Box<dyn UndoCommand<T>>>,
}

impl<T> Default for CommandHistory<T> {
    fn default() -> Self {
        Self {
            undos: Vec::new(),
            redos: Vec::new(),
        }
    }
}

impl<T> CommandHistory<T> {
    pub fn execute(&mut self, mut action: Box<dyn UndoCommand<T>>, state: &mut T) {
        self.redos.clear();

        action.execute(state);
        self.undos.push(action);
    }

    pub fn undo(&mut self, state: &mut T) {
        let Some(mut command) = self.undos.pop() else {
            return;
        };

        command.undo(state);
        self.redos.push(command);
    }

    pub fn redo(&mut self, state: &mut T) {
        let Some(mut command) = self.redos.pop() else {
            return;
        };

        command.undo(state);
        self.undos.push(command);
    }
}
