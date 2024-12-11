mod meta_state;

use crate::{graph::Graph, state::State};
use meta_state::MetaState;
use std::collections::HashSet;

const ALPHABET_SIZE: usize = 26;

/// [Iterator] over all the valid strings for a [Graph].
pub struct ProduceStrings<'a> {
    /// A reference to the graph for which we are producing strings.
    graph: &'a Graph,

    /// The maximum length of string we are producing.
    max_len: usize,

    /// The set of states corresponding to producing the current string, according to the referenced graph.
    current_states: HashSet<State>,

    /// The sequence of steps (according to the referenced graph) taken to get to the current state.
    ///
    /// Required for backtracking.
    meta_state_stack: Vec<(char, MetaState<'a>)>,
}

impl<'a> ProduceStrings<'a> {
    pub fn new(graph: &'a Graph, max_len: usize) -> Self {
        Self {
            graph,
            max_len,

            // the initial states includes epsilon transitions.
            current_states: graph.follow_epsilon_rules([graph.start].into()),

            // we know we will need at most `max_len` meta-states.
            meta_state_stack: Vec::with_capacity(max_len),
        }
    }

    /// Query whether the string corresponding to the current state matches the regex graph.
    fn is_in_matching_state(&self) -> bool {
        self.current_states.contains(&self.graph.end)
    }

    /// Query whether
    fn is_finished(&self) -> bool {
        self.current_states.is_empty() && self.meta_state_stack.is_empty()
    }

    fn next_impl(&mut self) {
        // create a new meta-state if we're not already at the max length.
        let mut new_meta_state = if self.meta_state_stack.len() < self.max_len {
            let states = self.current_states.drain().collect();
            Some(MetaState::new(self.graph, states))
        } else {
            None
        };

        loop {
            let current_meta_state = new_meta_state.take().or_else(|| {
                self.meta_state_stack
                    .pop()
                    .map(|(_, meta_state)| meta_state)
            });

            // this will either the new meta-state created above or one from the stack.
            let Some(mut current_meta_state) = current_meta_state else {
                // the stack is empty - iteration is complete.

                //
                self.current_states.clear();

                // the stack should have been emptied to get here.
                assert!(self.meta_state_stack.is_empty());

                return;
            };

            // advance the current meta-state, looking for a non-empty set of new states.
            while let Some((current_character, new_states)) = current_meta_state.next() {
                // if the new set of states is not empty, we found a valid (though not necessarily matching) state.
                if !new_states.is_empty() {
                    self.current_states = new_states;

                    self.meta_state_stack
                        .push((current_character, current_meta_state));

                    return;
                }
            }

            // the current meta-state is finished - backtrack and continue to the next loop iteration.

            // record the current meta-state's set of states as the new current states.
            self.current_states = current_meta_state.into_states();
        }
    }

    fn size_hint_upper(max_len: usize) -> Option<usize> {
        (0..max_len)
            .try_fold((1, 1), |(sum, alphabet_size_pow_n), _| {
                let alphabet_size_pow_n = ALPHABET_SIZE.checked_mul(alphabet_size_pow_n)?;

                let sum = alphabet_size_pow_n.checked_add(sum)?;

                Some((sum, alphabet_size_pow_n))
            })
            .map(|(sum, _)| sum)
    }
}

impl<'a> Iterator for ProduceStrings<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.is_in_matching_state() {
            // check for end of iteration.
            if self.is_finished() {
                return None;
            }

            self.next_impl();
        }

        let string = self
            .meta_state_stack
            .iter()
            .map(|(character, _)| character)
            .collect();

        self.next_impl();

        Some(string)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Self::size_hint_upper(self.max_len))
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::produce_strings::{ProduceStrings, ALPHABET_SIZE};

    fn size_hint_upper_recursive(max_len: usize) -> Option<usize> {
        if max_len == 0 {
            Some(1)
        } else {
            size_hint_upper_recursive(max_len - 1)?
                .checked_add(ALPHABET_SIZE.checked_pow(max_len.try_into().ok()?)?)
        }
    }

    #[test]
    fn test_size_hint() {
        for i in 0.. {
            let actual = ProduceStrings::size_hint_upper(i);
            let expected = size_hint_upper_recursive(i);

            assert_eq!(actual, expected);

            if expected.is_none() {
                break;
            }
        }
    }
}
