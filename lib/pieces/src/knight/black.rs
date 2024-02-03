use super::KnightExt;

use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
    }

    pub const fn symbol() -> char {
        'n'
    }

    pub const fn utf_symbol() -> char {
        '♞'
    }
}

impl Display for BlackKnight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackKnight::symbol())
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
