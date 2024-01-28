use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::KingExt;

#[derive(Debug)]
pub struct BlackKing {
    bitboard: Bitboard,
}

impl BlackKing {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait BlackKingExt: KingExt {}

impl BlackKingExt for BlackKing {}
impl KingExt for BlackKing {}
impl PieceExt for BlackKing {}
