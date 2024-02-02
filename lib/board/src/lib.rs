use fen::ForsythEdwardsNotation;
use pieces::Pieces;

pub static BOARD_SIZE: i8 = 8;
pub static NORTH: i8 = BOARD_SIZE;
pub static EAST: i8 = 1;
pub static SOUTH: i8 = -BOARD_SIZE;
pub static WEST: i8 = -EAST;
pub static NORTH_EAST: i8 = NORTH + EAST;
pub static SOUTH_EAST: i8 = SOUTH + EAST;
pub static SOUTH_WEST: i8 = SOUTH + WEST;
pub static NORTH_WEST: i8 = NORTH + WEST;
pub static NORTH_NORTH_EAST: i8 = NORTH + NORTH_EAST;
pub static NORTH_NORTH_WEST: i8 = NORTH + NORTH_WEST;
pub static SOUTH_SOUTH_EAST: i8 = SOUTH + SOUTH_EAST;
pub static SOUTH_SOUTH_WEST: i8 = SOUTH + SOUTH_WEST;
pub static EAST_EAST_NORTH: i8 = EAST + NORTH_EAST;
pub static EAST_EAST_SOUTH: i8 = EAST + SOUTH_EAST;
pub static WEST_WEST_NORTH: i8 = WEST + NORTH_WEST;
pub static WEST_WEST_SOUTH: i8 = WEST + SOUTH_WEST;

#[derive(Debug)]
pub struct Board {
    fen: ForsythEdwardsNotation,
    pieces: Pieces,
}

impl Board {
    pub fn new(fen: ForsythEdwardsNotation, pieces: Pieces) -> Self {
        Self { fen, pieces }
    }

    pub fn fen(&self) -> &ForsythEdwardsNotation {
        &self.fen
    }

    pub fn pieces(&self) -> &Pieces {
        &self.pieces
    }
}
