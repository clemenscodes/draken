use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::KnightExt;

#[derive(Debug)]
pub struct BlackKnight {
    bitboard: Bitboard,
}

impl BlackKnight {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait BlackKnightExt: KnightExt {}

impl BlackKnightExt for BlackKnight {}
impl KnightExt for BlackKnight {}
impl PieceExt for BlackKnight {}
