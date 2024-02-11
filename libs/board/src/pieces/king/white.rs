use super::{King, KingExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteKing {
    bitboard: Bitboard,
}

impl WhiteKing {
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
        'K'
    }

    pub const fn utf_symbol() -> char {
        '♔'
    }
}

impl From<Bitboard> for WhiteKing {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteKing::symbol())
    }
}

impl Debug for WhiteKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl KingExt for WhiteKing {}
impl PieceExt for WhiteKing {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        King::from(*self).is_illegal_move(source, destination, board)
    }
}

impl Verify for WhiteKing {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        King::from(*self).verify(source, destination, board)
    }
}
