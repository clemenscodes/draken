use super::KnightExt;
use crate::{Knight, March, PieceExt};
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

impl March for WhiteKnight {
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        Knight::from(*self).march(source, destination)
    }
}
