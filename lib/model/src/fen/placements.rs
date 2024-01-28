pub const NUM_FILES: usize = 8;
pub const NUM_RANKS: usize = 8;

type Rank = [char; NUM_FILES];

#[derive(Debug)]
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
