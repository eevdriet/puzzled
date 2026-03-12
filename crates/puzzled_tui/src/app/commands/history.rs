use crate::UndoAction;

pub struct ActionHistory<T> {
    undos: Vec<Box<dyn UndoAction<T>>>,
    redos: Vec<Box<dyn UndoAction<T>>>,
}

impl<T> Default for ActionHistory<T> {
    fn default() -> Self {
        Self {
            undos: Vec::new(),
            redos: Vec::new(),
        }
    }
}

impl<T> ActionHistory<T> {
    pub fn execute(&mut self, mut action: Box<dyn UndoAction<T>>, state: &mut T) {
        self.redos.clear();

        action.execute(state);
        self.undos.push(action);
    }

    pub fn undo(&mut self, count: usize, state: &mut T) {
        for _ in 0..count {
            let Some(mut command) = self.undos.pop() else {
                return;
            };

            command.undo(state);
            self.redos.push(command);
        }
    }

    pub fn redo(&mut self, count: usize, state: &mut T) {
        for _ in 0..count {
            let Some(mut command) = self.redos.pop() else {
                return;
            };

            command.undo(state);
            self.undos.push(command);
        }
    }
}
