use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::KnightExt;

#[derive(Debug)]
pub struct WhiteKnight {
    bitboard: Bitboard,
}

impl WhiteKnight {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait WhiteKnightExt: KnightExt {}

impl WhiteKnightExt for WhiteKnight {}
impl KnightExt for WhiteKnight {}
impl PieceExt for WhiteKnight {}
