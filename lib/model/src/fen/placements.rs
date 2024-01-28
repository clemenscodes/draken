use std::fmt::{Debug, Display};

pub const NUM_FILES: usize = 8;
pub const NUM_RANKS: usize = 8;

type Rank = [char; NUM_FILES];

pub struct Placements {
    placements: [Rank; NUM_RANKS],
}

impl Placements {
    pub fn new(placements: [Rank; NUM_RANKS]) -> Self {
        Self { placements }
    }

    pub fn placements(&self) -> [[char; NUM_FILES]; NUM_RANKS] {
        self.placements
    }
}

impl Display for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Debug for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
