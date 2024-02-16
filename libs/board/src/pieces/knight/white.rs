use super::{Knight, KnightExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteKnight {
    bitboard: Bitboard,
}

impl WhiteKnight {
    pub const SYMBOL: char = 'N';
    pub const UTF_SYMBOL: char = '♘';

    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
    }
}

impl From<Bitboard> for WhiteKnight {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteKnight::SYMBOL)
    }
}

impl Debug for WhiteKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for WhiteKnight {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Knight::from(*self).get_attacks(piece, board)
    }
}

impl Verify for WhiteKnight {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Knight::from(*self).verify(source, destination, promotion, board)
    }
}

impl KnightExt for WhiteKnight {}
