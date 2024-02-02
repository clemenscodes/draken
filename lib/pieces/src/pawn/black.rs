use super::PawnExt;
use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const BLACK_PAWN: char = 'p';

pub struct BlackPawn {
    bitboard: Bitboard,
}

impl BlackPawn {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

impl Display for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BLACK_PAWN}")
    }
}

impl Debug for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait BlackPawnExt: PawnExt {}

impl BlackPawnExt for BlackPawn {}
impl PawnExt for BlackPawn {}
impl PieceExt for BlackPawn {}
