pub const MAX_PLY: u8 = u8::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveList {
    moves: [u16; MAX_PLY as usize],
}

impl MoveList {
    pub fn new(moves: [u16; MAX_PLY as usize]) -> Self {
        Self { moves }
    }

    pub fn moves(&self) -> &[u16] {
        self.moves.as_ref()
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [0; MAX_PLY as usize],
        }
    }
}

pub trait MoveListExt {}

impl MoveListExt for MoveList {}
