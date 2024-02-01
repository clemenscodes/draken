use super::QueenExt;
use crate::{bitboard::Bitboard, pieces::PieceExt};
use std::fmt::{Debug, Display};

pub const WHITE_QUEEN: char = 'Q';

pub struct WhiteQueen {
    bitboard: Bitboard,
}

impl WhiteQueen {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

impl Display for WhiteQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{WHITE_QUEEN}")
    }
}

impl Debug for WhiteQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait WhiteQueenExt: QueenExt {}

impl WhiteQueenExt for WhiteQueen {}
impl QueenExt for WhiteQueen {}
impl PieceExt for WhiteQueen {}
