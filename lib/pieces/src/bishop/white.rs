use super::BishopExt;
use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default)]
pub struct WhiteBishop {
    bitboard: Bitboard,
}

impl WhiteBishop {
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
        'B'
    }

    pub const fn utf_symbol() -> char {
        '♗'
    }
}

impl Display for WhiteBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteBishop::symbol())
    }
}

impl Debug for WhiteBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait WhiteBishopExt: BishopExt {}

impl WhiteBishopExt for WhiteBishop {}
impl BishopExt for WhiteBishop {}
impl PieceExt for WhiteBishop {}
