use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::PawnExt;

#[derive(Debug)]
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
}

pub trait WhitePawnExt: PawnExt {}

impl WhitePawnExt for WhitePawn {}
impl PawnExt for WhitePawn {}
impl PieceExt for WhitePawn {}
