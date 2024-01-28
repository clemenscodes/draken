use std::fmt::{Debug, Display};

pub const MAX_HALF_MOVE_CLOCK: u8 = 150;
pub const MIN_HALF_MOVE_CLOCK: u8 = 0;

pub struct HalfMoveClock {
    clock: u8,
}

impl HalfMoveClock {
    pub fn new(clock: u8) -> Self {
        Self { clock }
    }

    pub fn clock(&self) -> u8 {
        self.clock
    }
}

impl Default for HalfMoveClock {
    fn default() -> Self {
        Self {
            clock: MIN_HALF_MOVE_CLOCK,
        }
    }
}

pub trait HalfMoveClockExt {
    fn increment(&mut self);
}

impl HalfMoveClockExt for HalfMoveClock {
    fn increment(&mut self) {
        if self.clock < MAX_HALF_MOVE_CLOCK {
            self.clock += 1;
        }
    }
}

impl Display for HalfMoveClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.clock)
    }
}

impl Debug for HalfMoveClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
