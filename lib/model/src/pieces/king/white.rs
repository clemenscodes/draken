use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::KingExt;

#[derive(Debug)]
pub struct WhiteKing {
    bitboard: Bitboard,
}

impl WhiteKing {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait WhiteKingExt: KingExt {}

impl WhiteKingExt for WhiteKing {}
impl KingExt for WhiteKing {}
impl PieceExt for WhiteKing {}
