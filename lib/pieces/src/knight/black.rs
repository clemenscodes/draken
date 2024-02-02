use super::KnightExt;

use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const BLACK_KNIGHT: char = 'n';

#[derive(Default)]
pub struct BlackKnight {
    bitboard: Bitboard,
}

impl BlackKnight {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

impl Display for BlackKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BLACK_KNIGHT}")
    }
}

impl Debug for BlackKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait BlackKnightExt: KnightExt {}

impl BlackKnightExt for BlackKnight {}
impl KnightExt for BlackKnight {}
impl PieceExt for BlackKnight {}
