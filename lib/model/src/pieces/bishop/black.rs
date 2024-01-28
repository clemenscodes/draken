use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::BishopExt;

#[derive(Debug)]
pub struct BlackBishop {
    bitboard: Bitboard,
}

impl BlackBishop {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait BlackBishopExt: BishopExt {}

impl BlackBishopExt for BlackBishop {}
impl BishopExt for BlackBishop {}
impl PieceExt for BlackBishop {}
