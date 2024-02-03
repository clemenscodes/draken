use super::QueenExt;
use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default)]
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

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
    }

    pub const fn symbol() -> char {
        'q'
    }

    pub const fn utf_symbol() -> char {
        '♛'
    }
}

impl Display for BlackQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackQueen::symbol())
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
