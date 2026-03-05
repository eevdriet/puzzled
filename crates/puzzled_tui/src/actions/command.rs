pub trait Command<T> {
    fn execute(&mut self, state: &mut T);
}

pub trait UndoCommand<T>: Command<T> {
    fn undo(&mut self, state: &mut T);

    fn redo(&mut self, state: &mut T) {
        self.execute(state);
    }
}
