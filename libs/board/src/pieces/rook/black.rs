use super::{Rook, RookExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackRook {
    bitboard: Bitboard,
}

impl BlackRook {
    pub const SYMBOL: char = 'r';
    pub const UTF_SYMBOL: char = '♜';

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

impl From<Bitboard> for BlackRook {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackRook::SYMBOL)
    }
}

impl Debug for BlackRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for BlackRook {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Rook::from(*self).get_attacks(piece, board)
    }
}

impl Verify for BlackRook {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Rook::from(*self).verify(source, destination, promotion, board)
    }
}

impl RookExt for BlackRook {}
