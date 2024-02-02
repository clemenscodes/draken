use super::QueenExt;
use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const BLACK_QUEEN: char = 'q';

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
}

impl Display for BlackQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BLACK_QUEEN}")
    }
}

impl Debug for BlackQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait BlackQueenExt: QueenExt {}

impl BlackQueenExt for BlackQueen {}
impl QueenExt for BlackQueen {}
impl PieceExt for BlackQueen {}
