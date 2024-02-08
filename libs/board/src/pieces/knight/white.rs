use super::{Knight, KnightExt};
use crate::{
    pieces::{PieceExt, Verify},
    Board,
};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteKnight {
    bitboard: Bitboard,
}

impl WhiteKnight {
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
        'N'
    }

    pub const fn utf_symbol() -> char {
        '♘'
    }
}

impl From<Bitboard> for WhiteKnight {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}
impl Display for WhiteKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteKnight::symbol())
    }
}

impl Debug for WhiteKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl KnightExt for WhiteKnight {}
impl PieceExt for WhiteKnight {}

impl Verify for WhiteKnight {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Knight::from(*self).verify(source, destination, board)
    }
}
