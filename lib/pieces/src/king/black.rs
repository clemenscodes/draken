use super::KingExt;

use crate::PieceExt;
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

pub trait BlackKingExt: KingExt {}

impl BlackKingExt for BlackKing {}
impl KingExt for BlackKing {}
impl PieceExt for BlackKing {}
