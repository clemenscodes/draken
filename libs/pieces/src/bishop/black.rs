use super::BishopExt;

use crate::{Bishop, March, PieceExt};
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackBishop {
    bitboard: Bitboard,
}

impl BlackBishop {
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
        'b'
    }

    pub const fn utf_symbol() -> char {
        '♝'
    }
}

impl From<Bitboard> for BlackBishop {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackBishop::symbol())
    }
}

impl Debug for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl BishopExt for BlackBishop {}
impl PieceExt for BlackBishop {}

impl March for BlackBishop {
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        Bishop::from(*self).march(source, destination)
    }
}
