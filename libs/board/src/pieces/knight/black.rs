use super::{Knight, KnightExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackKnight {
    bitboard: Bitboard,
}

impl BlackKnight {
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
        'n'
    }

    pub const fn utf_symbol() -> char {
        '♞'
    }
}

impl From<Bitboard> for BlackKnight {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackKnight::symbol())
    }
}

impl Debug for BlackKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for BlackKnight {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        Knight::from(*self).is_illegal_move(source, destination, board)
    }

    fn get_attacks(&self, piece: Bitboard, board: &mut Board) -> Bitboard {
        Knight::from(*self).get_attacks(piece, board)
    }
}

impl Verify for BlackKnight {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Knight::from(*self).verify(source, destination, board)
    }
}

impl KnightExt for BlackKnight {}
