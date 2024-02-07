use super::{Queen, QueenExt};
use crate::{
    pieces::{March, PieceExt},
    Board,
};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackQueen {
    bitboard: Bitboard,
}

impl BlackQueen {
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
        'q'
    }

    pub const fn utf_symbol() -> char {
        '♛'
    }
}

impl From<Bitboard> for BlackQueen {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}
impl Display for BlackQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackQueen::symbol())
    }
}

impl Debug for BlackQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueenExt for BlackQueen {}
impl PieceExt for BlackQueen {}

impl March for BlackQueen {
    fn march(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Queen::from(*self).march(source, destination, board)
    }
}
