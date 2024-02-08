use super::{Rook, RookExt};
use crate::{
    pieces::{PieceExt, Verify},
    Board,
};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteRook {
    bitboard: Bitboard,
}

impl WhiteRook {
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
        'R'
    }

    pub const fn utf_symbol() -> char {
        '♖'
    }
}

impl From<Bitboard> for WhiteRook {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteRook::symbol())
    }
}

impl Debug for WhiteRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl RookExt for WhiteRook {}
impl PieceExt for WhiteRook {}

impl Verify for WhiteRook {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Rook::from(*self).verify(source, destination, board)
    }
}