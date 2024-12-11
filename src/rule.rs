use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    mem::discriminant,
};

use crate::state::State;

#[derive(Hash)]
pub struct Rule {
    start: State,
    end: State,
    matcher: Matcher,
}

impl Rule {
    pub fn epsilon(start: State, end: State) -> Self {
        Self {
            start,
            end,
            matcher: Matcher::Epsilon { name: None },
        }
    }

    pub fn named_epsilon(start: State, end: State, name: String) -> Self {
        Self {
            start,
            end,
            matcher: Matcher::Epsilon { name: Some(name) },
        }
    }

    pub fn lambda<F>(start: State, end: State, name: String, lambda: F) -> Self
    where
        F: Fn(char) -> bool + 'static,
    {
        Self {
            start,
            end,
            matcher: Matcher::Lambda {
                lambda: Box::new(lambda),
                name,
            },
        }
    }

    pub fn match_eq(start: State, end: State, character_to_match: char) -> Self {
        Self::lambda(
            start,
            end,
            format!("EQ {character_to_match}"),
            move |character| character_to_match == character,
        )
    }

    pub fn match_any(start: State, end: State) -> Self {
        Self::lambda(start, end, "ANY".to_owned(), |_| true)
    }

    pub fn start(&self) -> State {
        self.start
    }

    pub fn end(&self) -> State {
        self.end
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(self.matcher, Matcher::Epsilon { .. })
    }

    pub fn matches(&self, character: char) -> bool {
        if let Matcher::Lambda { lambda, .. } = &self.matcher {
            lambda(character)
        } else {
            false
        }
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Rule {}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{:?} -> {:?} [label=\"{:?}\"]",
            self.start, self.end, self.matcher
        )
    }
}

enum Matcher {
    Lambda {
        lambda: Box<dyn Fn(char) -> bool>,
        name: String,
    },
    Epsilon {
        name: Option<String>,
    },
}

impl Hash for Matcher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
        if let Self::Lambda { name, .. } = self {
            name.hash(state);
        }
    }
}

impl Debug for Matcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Lambda { name, .. } => write!(f, "{name}"),
            Self::Epsilon { name: Some(name) } => write!(f, "{} (ε)", name),
            _ => write!(f, "ε"),
        }
    }
}
