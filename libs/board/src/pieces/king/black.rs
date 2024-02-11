use super::{King, KingExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackKing {
    bitboard: Bitboard,
}

impl BlackKing {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
    }

    pub const fn symbol() -> char {
        'k'
    }

    pub const fn utf_symbol() -> char {
        '♚'
    }
}

impl From<Bitboard> for BlackKing {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackKing::symbol())
    }
}

impl Debug for BlackKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl KingExt for BlackKing {}
impl PieceExt for BlackKing {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        King::from(*self).is_illegal_move(source, destination, board)
    }
}

impl Verify for BlackKing {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        King::from(*self).verify(source, destination, board)
    }
}
