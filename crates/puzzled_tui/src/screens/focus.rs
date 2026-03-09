use std::{collections::HashMap, hash::Hash};

use puzzled_core::Direction;

use crate::{Action, ActionResolver, Command, HandleCommand};

pub struct FocusManager<F> {
    curr: F,
    links: HashMap<F, [Option<F>; 4]>,
}

impl<F> FocusManager<F> {
    pub fn new(curr: F) -> Self {
        Self {
            curr,
            links: HashMap::default(),
        }
    }

    pub fn current(&self) -> &F {
        &self.curr
    }
}

impl<F> Default for FocusManager<F>
where
    F: Default,
{
    fn default() -> Self {
        Self {
            curr: F::default(),
            links: HashMap::default(),
        }
    }
}

impl<F> FocusManager<F>
where
    F: Eq + Hash + Copy,
{
    pub fn link_left(&mut self, middle: F, left: &[F]) {
        self.link_in_direction(middle, left, Direction::Right);
    }

    pub fn link_right(&mut self, middle: F, right: &[F]) {
        self.link_in_direction(middle, right, Direction::Left);
    }

    pub fn link_above(&mut self, middle: F, above: &[F]) {
        self.link_in_direction(middle, above, Direction::Down);
    }

    pub fn link_below(&mut self, middle: F, below: &[F]) {
        self.link_in_direction(middle, below, Direction::Up);
    }

    fn link_in_direction(&mut self, middle: F, others: &[F], direction: Direction) {
        for other in others {
            {
                let middle_links = self.links.entry(middle).or_default();
                middle_links[!direction as usize] = Some(*other);
            }

            {
                let other_links = self.links.entry(*other).or_default();
                other_links[direction as usize] = Some(middle);
            }
        }
    }
}

impl<M, A, T, F> HandleCommand<M, A, T> for FocusManager<F>
where
    F: Eq + Hash + Copy,
{
    type State = ();

    fn on_command(
        &mut self,
        command: Command<M, A>,
        _resolver: ActionResolver<M, A, T>,
        _state: &mut Self::State,
    ) -> bool {
        // Make sure focus can be given up from the current node
        let Some(links) = self.links.get(&self.curr) else {
            return false;
        };

        // Make sure a focus action is given
        let Some(action) = command.action() else {
            return false;
        };

        // Determine which node to focus next and focus if found
        let next = match action {
            Action::FocusUp => links[Direction::Up as usize],
            Action::FocusRight => links[Direction::Right as usize],
            Action::FocusDown => links[Direction::Down as usize],
            Action::FocusLeft => links[Direction::Left as usize],
            _ => return false,
        };

        if let Some(next) = next {
            self.curr = next;
        }

        true
    }
}
