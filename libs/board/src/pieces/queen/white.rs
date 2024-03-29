use super::{Queen, QueenExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteQueen {
    bitboard: Bitboard,
}

impl WhiteQueen {
    pub const SYMBOL: char = 'Q';
    pub const UTF_SYMBOL: char = '♕';

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

impl From<Bitboard> for WhiteQueen {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteQueen::SYMBOL)
    }
}

impl Debug for WhiteQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for WhiteQueen {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Queen::from(*self).get_attacks(piece, board)
    }
}

impl Verify for WhiteQueen {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Queen::from(*self).verify(source, destination, promotion, board)
    }
}

impl QueenExt for WhiteQueen {}
