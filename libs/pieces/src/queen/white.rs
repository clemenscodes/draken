use super::QueenExt;
use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
    }

    pub const fn symbol() -> char {
        'Q'
    }

    pub const fn utf_symbol() -> char {
        '♕'
    }
}

impl From<Bitboard> for WhiteQueen {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteQueen::symbol())
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
