pub trait Solve {
    type Value: Clone + Eq;
    type Position;

    fn solve(&mut self, pos: &Self::Position, solution: Self::Value) -> bool;
    fn enter(&mut self, pos: &Self::Position, solution: Self::Value) -> bool;
    fn reveal(&mut self, pos: &Self::Position) -> bool;
    fn check(&mut self, pos: &Self::Position) -> Option<bool>;

    fn reveal_all(&mut self);
    fn check_all(&mut self);

    fn enter_checked(&mut self, pos: &Self::Position, solution: Self::Value) -> Option<bool> {
        self.enter(pos, solution);
        self.check(pos)
    }
}
