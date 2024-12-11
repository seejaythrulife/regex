use self::{evaluate::Evaluate, produce_strings::ProduceStrings};
use crate::{rule::Rule, state::State};
use std::{
    collections::HashSet,
    fmt::{Debug, Formatter, Result as FmtResult},
    iter::empty,
};

pub mod evaluate;
pub mod produce_strings;

pub struct Graph {
    label: String,
    start: State,
    end: State,
    rules: Vec<Rule>,
}

impl Graph {
    fn new(label: String, start: State, end: State, rules: impl IntoIterator<Item = Rule>) -> Self {
        Self {
            label,
            start,
            end,
            rules: rules.into_iter().collect(),
        }
    }

    pub fn any() -> Self {
        let label = ".".to_owned();

        let start = State::new();
        let end = State::new();

        let rules = [
            // add lambda rule matching any character.
            Rule::match_any(start, end),
        ];

        Self::new(label, start, end, rules)
    }

    #[must_use]
    pub fn zero_or_more(self) -> Self {
        let label = format!("({})*", self.label);

        let start = State::new();
        let end = State::new();

        let rules = empty()
            // include all rules in self.
            .chain(self.rules)
            .chain([
                // add rule skipping self.
                Rule::epsilon(start, end),
                // add path through self.
                Rule::epsilon(start, self.start),
                Rule::epsilon(self.end, end),
                // add path repeating self.
                Rule::epsilon(self.end, self.start),
            ]);

        Self::new(label, start, end, rules)
    }

    #[must_use]
    pub fn one_or_more(self) -> Self {
        let label = format!("({})+", self.label);

        let start = State::new();
        let end = State::new();

        let rules = empty()
            // include all rules in self.
            .chain(self.rules)
            .chain([
                // add path into self.
                Rule::epsilon(start, self.start),
                // add path out of self.
                Rule::epsilon(self.end, end),
                // add path repeating self.
                Rule::epsilon(self.end, self.start),
            ]);

        Self::new(label, start, end, rules)
    }

    #[must_use]
    pub fn optional(self) -> Self {
        let label = format!("({})?", self.label);

        let start = State::new();
        let end = State::new();

        let rules = empty()
            // include all rules in self.
            .chain(self.rules)
            .chain([
                // add path skipping self.
                Rule::epsilon(start, end),
                // add path through self.
                Rule::epsilon(start, self.start),
                Rule::epsilon(self.end, end),
            ]);

        Self::new(label, start, end, rules)
    }

    #[must_use]
    pub fn concat(self, other: Self) -> Self {
        let label = format!("{}{}", self.label, other.label);

        let start = self.start;
        let end = other.end;

        let rules = empty()
            // include all rules in self.
            .chain(self.rules)
            // include all rules in other.
            .chain(other.rules)
            .chain([
                // add path from self to other.
                Rule::epsilon(self.end, other.start),
            ]);

        Self::new(label, start, end, rules)
    }

    #[must_use]
    pub fn concat_many<const N: usize>(self, others: [Self; N]) -> Self {
        others.into_iter().fold(self, Self::concat)
    }

    #[must_use]
    pub fn union(self, other: Self) -> Self {
        let label = format!("({}|{})", self.label, other.label);

        let start = State::new();
        let end = State::new();

        let rules = empty()
            // include all rules in self.
            .chain(self.rules)
            // include all rules in other.
            .chain(other.rules)
            .chain([
                // add path through self.
                Rule::epsilon(start, self.start),
                Rule::epsilon(self.end, end),
                // add path through other.
                Rule::epsilon(start, other.start),
                Rule::epsilon(other.end, end),
            ]);

        Self::new(label, start, end, rules)
    }

    /// The label of the [Graph]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn start(&self) -> &State {
        &self.start
    }

    pub fn end(&self) -> &State {
        &self.end
    }

    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
    }

    /// Query whether the [Graph] is able to match the string.
    pub fn matches(&self, string_to_match: &str) -> bool {
        string_to_match
            // for each character in the string...
            .chars()
            // ... iterate an evaluator initially in the starting state, following the rules for each character in `string` until either:
            //  we processed the whole string; or we produce an evaluator in zero states
            .try_fold(Evaluate::new(self), Evaluate::try_follow_rules)
            // we may or may not produced a valid evaluator, which may or may not be in the end state.
            .is_some_and(|result| result.is_in_end_state())
    }

    pub fn produce_strings<'a>(&'a self, max_len: usize) -> ProduceStrings<'a> {
        ProduceStrings::new(self, max_len)
    }

    /// Get the set of states reachable by any number of epsilon rules (including zero) in [Graph]'s, starting from any state in the starting states.
    fn follow_epsilon_rules(&self, start_states: HashSet<State>) -> HashSet<State> {
        fn follow_epsilons_impl(rules: Vec<&Rule>, states: HashSet<State>) -> HashSet<State> {
            // filter for rules that end outside `states`.
            let rules: Vec<_> = rules
                .into_iter()
                .filter(|rule| !states.contains(&rule.end()))
                .collect();

            let next_states: HashSet<_> = rules
                .iter()
                // filter for rules that start inside `states`.
                .filter(|rule| states.contains(&rule.start()))
                // map rules to end states.
                .map(|rule| rule.end())
                .collect();

            if next_states.is_empty() {
                states
            } else {
                states
                    .into_iter()
                    .chain(follow_epsilons_impl(rules, next_states))
                    .collect()
            }
        }

        let starting_rules = self
            .rules
            .iter()
            // we only want epsilon rules.
            .filter(|rule| rule.is_epsilon())
            .collect();

        follow_epsilons_impl(starting_rules, start_states)
    }

    fn follow_rules(&self, start_states: &HashSet<State>, character: char) -> HashSet<State> {
        if start_states.is_empty() {
            HashSet::new()
        } else {
            let end_states = self
                .rules
                .iter()
                // filter for rules that match the current character.
                .filter(|rule| rule.matches(character))
                // filter for rules that start inside the set of start states.
                .filter(|rule| start_states.contains(&rule.start()))
                // get the set of states reachable from the end state of each rule
                .map(|rule| rule.end())
                .collect();

            self.follow_epsilon_rules(end_states)
        }
    }
}

impl From<char> for Graph {
    fn from(character: char) -> Self {
        let start = State::new();
        let end = State::new();

        Self::new(
            character.to_string(),
            start,
            end,
            [
                // add lambda rule matching the character.
                Rule::match_eq(start, end, character),
            ],
        )
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "digraph \"{}\" {{\n", self.label)?;

        write!(f, "{:?} [shape=square];\n", self.start)?;
        write!(f, "{:?} [shape=doublecircle];\n", self.end)?;

        for rule in &self.rules {
            write!(f, "{:?};\n", rule)?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}
