use crate::moves::Move;

#[derive(Debug)]
pub struct MoveList {
    moves: Vec<Move>,
}

impl MoveList {
    pub fn new(moves: Vec<Move>) -> Self {
        Self { moves }
    }

    pub fn moves(&self) -> &[Move] {
        self.moves.as_ref()
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: Default::default(),
        }
    }
}

pub trait MoveListExt {}

impl MoveListExt for MoveList {}
