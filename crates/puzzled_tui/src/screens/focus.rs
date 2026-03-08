use std::{collections::HashMap, hash::Hash};

use puzzled_core::Direction;

use crate::{Action, HandleAction};

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
    F: Eq + Hash + Clone,
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

    fn link_in_direction(&mut self, middle: F, other: &[F], direction: Direction) {
        for focus in other {
            let links = self.links.entry(focus.clone()).or_default();
            links[direction as usize] = Some(middle.clone());
        }
    }
}

impl<A, T, F> HandleAction<A, T> for FocusManager<F>
where
    F: Eq + Hash + Clone,
{
    type State = ();

    fn on_action(
        &mut self,
        action: crate::Action<A>,
        _resolver: crate::ActionResolver<A, T>,
        _state: &mut Self::State,
    ) {
        let Some(links) = self.links.get(&self.curr) else {
            return;
        };

        let next = match action {
            Action::FocusUp => links[Direction::Up as usize].clone(),
            Action::FocusRight => links[Direction::Right as usize].clone(),
            Action::FocusDown => links[Direction::Down as usize].clone(),
            Action::FocusLeft => links[Direction::Left as usize].clone(),
            _ => None,
        };

        if let Some(next) = next {
            self.curr = next;
        }
    }
}
