use super::{Rook, RookExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteRook {
    bitboard: Bitboard,
}

impl WhiteRook {
    pub const SYMBOL: char = 'R';
    pub const UTF_SYMBOL: char = '♖';

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

impl From<Bitboard> for WhiteRook {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteRook::SYMBOL)
    }
}

impl Debug for WhiteRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for WhiteRook {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Rook::from(*self).get_attacks(piece, board)
    }
}

impl Verify for WhiteRook {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Rook::from(*self).verify(source, destination, promotion, board)
    }
}

impl RookExt for WhiteRook {}
