use crate::PieceExt;

use super::RookExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteRook {
    bitboard: Bitboard,
}

impl WhiteRook {
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
        'R'
    }

    pub const fn utf_symbol() -> char {
        '♖'
    }
}

impl From<Bitboard> for WhiteRook {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteRook::symbol())
    }
}

impl Debug for WhiteRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait WhiteRookExt: RookExt {}

impl WhiteRookExt for WhiteRook {}
impl RookExt for WhiteRook {}
impl PieceExt for WhiteRook {}
