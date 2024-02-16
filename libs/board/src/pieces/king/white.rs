use super::{King, KingExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteKing {
    bitboard: Bitboard,
}

impl WhiteKing {
    pub const SYMBOL: char = 'K';
    pub const UTF_SYMBOL: char = '♔';

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

impl From<Bitboard> for WhiteKing {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteKing::SYMBOL)
    }
}

impl Debug for WhiteKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for WhiteKing {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        King::from(*self).get_attacks(piece, board)
    }
}

impl Verify for WhiteKing {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        King::from(*self).verify(source, destination, promotion, board)
    }
}

impl KingExt for WhiteKing {}
