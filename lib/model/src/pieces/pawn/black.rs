use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::PawnExt;

#[derive(Debug)]
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

pub trait BlackPawnExt: PawnExt {}

impl BlackPawnExt for BlackPawn {}
impl PawnExt for BlackPawn {}
impl PieceExt for BlackPawn {}
