use std::{collections::HashMap, hash::Hash};

use puzzled_core::Direction;

use crate::{
    Action, ActionBehavior, AppCommand, AppContext, AppResolver, AppTypes, Command, HandleCommand,
};

#[derive(Debug, Clone, Copy)]
pub struct FocusNode<F> {
    links: [Option<F>; 4],
}

impl<F> Default for FocusNode<F> {
    fn default() -> Self {
        Self {
            links: [None, None, None, None],
        }
    }
}

pub struct FocusManager<F> {
    curr: F,
    graph: HashMap<F, FocusNode<F>>,
}

impl<F> Default for FocusManager<F>
where
    F: Default,
{
    fn default() -> Self {
        Self {
            curr: F::default(),
            graph: HashMap::default(),
        }
    }
}

impl<F> FocusManager<F> {
    pub fn new(curr: F) -> Self {
        Self {
            curr,
            graph: HashMap::new(),
        }
    }

    pub fn get(&self) -> &F {
        &self.curr
    }

    pub fn set(&mut self, focus: F) {
        self.curr = focus;
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
                let node = self.graph.entry(middle).or_default();
                node.links[!direction as usize] = Some(*other);
            }

            {
                let node = self.graph.entry(*other).or_default();
                node.links[direction as usize] = Some(middle);
            }
        }
    }
}

impl<F, A> HandleCommand<A> for FocusManager<F>
where
    A: AppTypes,
    F: Eq + Hash + Copy,
{
    type State = ();

    fn handle_command(
        &mut self,
        command: AppCommand<A>,
        _resolver: AppResolver<A>,
        _ctx: &mut AppContext<A>,
        _state: &mut Self::State,
    ) -> bool {
        let Command::Action { action, .. } = command else {
            return false;
        };

        if !action.is_focus() {
            return false;
        }

        // Make sure focus can be given up from the current node
        let Some(node) = self.graph.get(&self.curr) else {
            return false;
        };

        // Determine which node to focus next and focus if found
        let next = match action {
            Action::FocusUp => node.links[Direction::Up as usize],
            Action::FocusRight => node.links[Direction::Right as usize],
            Action::FocusDown => node.links[Direction::Down as usize],
            Action::FocusLeft => node.links[Direction::Left as usize],
            _ => return false,
        };

        let Some(next) = next else {
            return false;
        };

        self.curr = next;
        true
    }
}
