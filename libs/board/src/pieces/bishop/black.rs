use super::{Bishop, BishopExt, PieceExt};
use crate::{Board, Verify};
use api::Square;
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

impl PieceExt for BlackBishop {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        Bishop::from(*self).is_illegal_move(source, destination, board)
    }

    fn get_attacks(&self, piece: Bitboard, board: &mut Board) -> Bitboard {
        Bishop::from(*self).get_attacks(piece, board)
    }
}

impl Verify for BlackBishop {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Bishop::from(*self).verify(source, destination, board)
    }
}

impl BishopExt for BlackBishop {}
