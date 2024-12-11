use super::{Graph, State};
use std::collections::HashSet;

pub struct Evaluate<'a> {
    graph: &'a Graph,
    current_states: HashSet<State>,
}

impl<'a> Evaluate<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            current_states: graph.follow_epsilon_rules([graph.start].into()),
        }
    }

    pub fn state_count(&self) -> usize {
        self.current_states.len()
    }

    pub fn is_in_end_state(&self) -> bool {
        self.current_states.contains(&self.graph.end)
    }

    pub fn current_states<'b>(&'b self) -> &'b HashSet<State> {
        &self.current_states
    }

    pub fn try_follow_rules(self, character: char) -> Option<Self> {
        let next = self.graph.follow_rules(&self.current_states, character);

        if next.is_empty() {
            None
        } else {
            Some(Self {
                graph: self.graph,
                current_states: next,
            })
        }
    }
}
