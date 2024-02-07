use super::PawnExt;
use crate::{March, Pawn, PieceExt};
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackPawn {
    bitboard: Bitboard,
}

impl BlackPawn {
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
        'p'
    }

    pub const fn utf_symbol() -> char {
        '♟'
    }
}

impl From<Bitboard> for BlackPawn {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}
impl Display for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackPawn::symbol())
    }
}

impl Debug for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PawnExt for BlackPawn {}
impl PieceExt for BlackPawn {}

impl March for BlackPawn {
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        Pawn::from(*self).march(source, destination)
    }
}
