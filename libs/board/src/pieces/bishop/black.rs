use super::{Bishop, BishopExt, PieceExt};
use crate::{Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackBishop {
    bitboard: Bitboard,
}

impl BlackBishop {
    pub const SYMBOL: char = 'b';
    pub const UTF_SYMBOL: char = '♝';

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

impl From<Bitboard> for BlackBishop {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackBishop::SYMBOL)
    }
}

impl Debug for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for BlackBishop {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Bishop::from(*self).get_attacks(piece, board)
    }
}

impl Verify for BlackBishop {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Bishop::from(*self).verify(source, destination, promotion, board)
    }
}

impl BishopExt for BlackBishop {}
