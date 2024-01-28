use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::BishopExt;

#[derive(Debug)]
pub struct WhiteBishop {
    bitboard: Bitboard,
}

impl WhiteBishop {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait WhiteBishopExt: BishopExt {}

impl WhiteBishopExt for WhiteBishop {}
impl BishopExt for WhiteBishop {}
impl PieceExt for WhiteBishop {}
