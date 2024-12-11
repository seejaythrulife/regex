use super::{Graph, State};
use std::{collections::HashSet, ops::RangeInclusive};

/// [Iterator] over the [State]s reachable from a set of initial states by matching a single character.
pub(super) struct MetaState<'a> {
    graph: &'a Graph,
    characters: RangeInclusive<char>,
    states: HashSet<State>,
}

impl<'a> MetaState<'a> {
    pub fn new(graph: &'a Graph, states: HashSet<State>) -> Self {
        Self {
            graph,
            characters: 'a'..='z',
            states,
        }
    }

    pub fn into_states(self) -> HashSet<State> {
        self.states
    }

    fn follow_rules(&self, character: char) -> HashSet<State> {
        self.graph.follow_rules(&self.states, character)
    }
}

impl<'a> Iterator for MetaState<'a> {
    type Item = (char, HashSet<State>);

    fn next(&mut self) -> Option<Self::Item> {
        self.characters
            .next()
            .map(|character| (character, self.follow_rules(character)))
    }
}
