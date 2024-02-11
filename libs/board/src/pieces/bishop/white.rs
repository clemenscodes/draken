use super::{Bishop, BishopExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteBishop {
    bitboard: Bitboard,
}

impl WhiteBishop {
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
        'B'
    }

    pub const fn utf_symbol() -> char {
        '♗'
    }
}

impl From<Bitboard> for WhiteBishop {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteBishop::symbol())
    }
}

impl Debug for WhiteBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for WhiteBishop {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        Bishop::from(*self).is_illegal_move(source, destination, board)
    }
}

impl Verify for WhiteBishop {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Bishop::from(*self).verify(source, destination, board)
    }
}

impl BishopExt for WhiteBishop {}
