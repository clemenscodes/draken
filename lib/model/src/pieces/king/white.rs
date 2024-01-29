use super::KingExt;
use crate::{bitboard::Bitboard, pieces::PieceExt};
use std::fmt::{Debug, Display};

pub const WHITE_KING: char = 'K';

pub struct WhiteKing {
    bitboard: Bitboard,
}

impl WhiteKing {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

impl Display for WhiteKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{WHITE_KING}")
    }
}

impl Debug for WhiteKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait WhiteKingExt: KingExt {}

impl WhiteKingExt for WhiteKing {}
impl KingExt for WhiteKing {}
impl PieceExt for WhiteKing {}
