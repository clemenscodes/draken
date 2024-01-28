pub const NUM_FILES: usize = 8;
pub const NUM_RANKS: usize = 8;

type Rank = [char; NUM_FILES];

#[derive(Debug)]
pub struct Placements {
    placements: [Rank; NUM_RANKS],
}
