use std::fmt::{Debug, Display};

pub const MAX_PLY: usize = 512;

type Moves = [u16; MAX_PLY];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MoveList {
    moves: Moves,
}

impl MoveList {
    pub fn new(moves: Moves) -> Self {
        Self { moves }
    }

    pub fn moves(&self) -> &[u16] {
        self.moves.as_ref()
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self { moves: [0; MAX_PLY] }
    }
}

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Move list!")
    }
}

impl Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait MoveListExt {}

impl MoveListExt for MoveList {}
