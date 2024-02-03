use super::BishopExt;

use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackBishop {
    bitboard: Bitboard,
}

impl BlackBishop {
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
        'b'
    }

    pub const fn utf_symbol() -> char {
        '♝'
    }
}

impl Display for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackBishop::symbol())
    }
}

impl Debug for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait BlackBishopExt: BishopExt {}

impl BlackBishopExt for BlackBishop {}
impl BishopExt for BlackBishop {}
impl PieceExt for BlackBishop {}
