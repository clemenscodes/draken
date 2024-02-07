use crate::{rook::RookExt, March};
use crate::{PieceExt, Rook};
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackRook {
    bitboard: Bitboard,
}

impl BlackRook {
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
        'r'
    }

    pub const fn utf_symbol() -> char {
        '♜'
    }
}

impl From<Bitboard> for BlackRook {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}
impl Display for BlackRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackRook::symbol())
    }
}

impl Debug for BlackRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait BlackRookExt: RookExt {}

impl RookExt for BlackRook {}
impl PieceExt for BlackRook {}

impl March for BlackRook {
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        Rook::from(*self).march(source, destination)
    }
}
