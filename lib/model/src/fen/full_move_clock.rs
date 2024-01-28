use std::fmt::{Debug, Display};

pub const MIN_FULL_MOVE_CLOCK: u16 = 1;

pub struct FullMoveClock {
    clock: u16,
}

impl FullMoveClock {
    pub fn new(clock: u16) -> Self {
        Self { clock }
    }

    pub fn clock(&self) -> u16 {
        self.clock
    }
}

impl Default for FullMoveClock {
    fn default() -> Self {
        Self {
            clock: MIN_FULL_MOVE_CLOCK,
        }
    }
}

pub trait FullMoveClockExt {
    fn increment(&mut self);
}

impl FullMoveClockExt for FullMoveClock {
    fn increment(&mut self) {
        self.clock += 1;
    }
}

impl Display for FullMoveClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clock)
    }
}

impl Debug for FullMoveClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
