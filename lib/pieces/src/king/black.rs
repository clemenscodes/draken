use super::KingExt;

use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const BLACK_KING: char = 'k';

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
}

impl Display for BlackKing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BLACK_KING}")
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
