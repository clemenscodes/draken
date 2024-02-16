use super::{Queen, QueenExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackQueen {
    bitboard: Bitboard,
}

impl BlackQueen {
    pub const SYMBOL: char = 'q';
    pub const UTF_SYMBOL: char = '♛';

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

impl From<Bitboard> for BlackQueen {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackQueen::SYMBOL)
    }
}

impl Debug for BlackQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for BlackQueen {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Queen::from(*self).get_attacks(piece, board)
    }
}

impl Verify for BlackQueen {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Queen::from(*self).verify(source, destination, promotion, board)
    }
}

impl QueenExt for BlackQueen {}
