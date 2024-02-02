use super::KnightExt;
use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const WHITE_KNIGHT: char = 'N';

#[derive(Default)]
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

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
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
