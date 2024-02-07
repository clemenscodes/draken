use super::KingExt;

use crate::{King, March, PieceExt};
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackKing {
    bitboard: Bitboard,
}

impl BlackKing {
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
        'k'
    }

    pub const fn utf_symbol() -> char {
        '♚'
    }
}

impl From<Bitboard> for BlackKing {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackKing::symbol())
    }
}

impl Debug for BlackKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl KingExt for BlackKing {}
impl PieceExt for BlackKing {}

impl March for BlackKing {
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        King::from(*self).march(source, destination)
    }
}
