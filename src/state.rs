use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct State(u32);

impl State {
    fn get_next_id() -> u32 {
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn new() -> Self {
        Self(Self::get_next_id())
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "S{}", self.0)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}
