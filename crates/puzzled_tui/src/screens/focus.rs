use std::{collections::HashMap, hash::Hash};

use puzzled_core::Direction;

use crate::{Action, ActionResolver, Command, EventMode, HandleCommand};

#[derive(Debug, Clone, Copy)]
pub struct FocusNode<F> {
    links: [Option<F>; 4],
    mode: Option<EventMode>,
}

impl<F> Default for FocusNode<F> {
    fn default() -> Self {
        Self {
            links: [None, None, None, None],
            mode: None,
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

    pub fn from_mode_nodes<M>(nodes: M) -> Self
    where
        F: Default + Eq + Hash,
        M: Into<HashMap<F, EventMode>>,
    {
        let mut graph = HashMap::default();
        let nodes = nodes.into();

        for (focus, mode) in nodes {
            let node: &mut FocusNode<F> = graph.entry(focus).or_default();
            node.mode = Some(mode);
        }

        Self {
            curr: F::default(),
            graph,
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

impl<M, A, T, F> HandleCommand<M, A, T> for FocusManager<F>
where
    F: Eq + Hash + Copy,
{
    type State = EventMode;

    fn on_command(
        &mut self,
        command: Command<M, A>,
        _resolver: ActionResolver<M, A, T>,
        mode: &mut Self::State,
    ) -> bool {
        // Make sure focus can be given up from the current node
        let Some(node) = self.graph.get(&self.curr) else {
            return false;
        };

        // Make sure a focus action is given
        let Some(action) = command.action() else {
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

        // Also set the entering mode if one is defined
        if let Some(next_node) = self.graph.get(&next)
            && let Some(next_mode) = next_node.mode
        {
            *mode = next_mode;
        }

        true
    }
}
