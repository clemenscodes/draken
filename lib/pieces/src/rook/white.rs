use crate::PieceExt;

use super::RookExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const WHITE_ROOK: char = 'R';

#[derive(Default)]
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
}

impl Display for WhiteRook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{WHITE_ROOK}")
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
