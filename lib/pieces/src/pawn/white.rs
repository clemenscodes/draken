use super::PawnExt;

use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const WHITE_PAWN: char = 'P';

#[derive(Default)]
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
}

impl Display for WhitePawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{WHITE_PAWN}")
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
