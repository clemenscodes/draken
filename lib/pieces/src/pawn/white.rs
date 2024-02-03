use super::PawnExt;

use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhitePawn {
    bitboard: Bitboard,
}

impl WhitePawn {
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
        'P'
    }

    pub const fn utf_symbol() -> char {
        '♙'
    }
}

impl Display for WhitePawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhitePawn::symbol())
    }
}

impl Debug for WhitePawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait WhitePawnExt: PawnExt {}

impl WhitePawnExt for WhitePawn {}
impl PawnExt for WhitePawn {}
impl PieceExt for WhitePawn {}
