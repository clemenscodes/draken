use super::KnightExt;
use crate::{bitboard::Bitboard, pieces::PieceExt};
use std::fmt::{Debug, Display};

pub const WHITE_KNIGHT: char = 'N';

pub struct WhiteKnight {
    bitboard: Bitboard,
}

impl WhiteKnight {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

impl Display for WhiteKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{WHITE_KNIGHT}")
    }
}

impl Debug for WhiteKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait WhiteKnightExt: KnightExt {}

impl WhiteKnightExt for WhiteKnight {}
impl KnightExt for WhiteKnight {}
impl PieceExt for WhiteKnight {}
