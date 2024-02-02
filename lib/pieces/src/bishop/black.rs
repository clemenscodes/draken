use super::BishopExt;

use crate::PieceExt;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

pub const BLACK_BISHOP: char = 'b';

#[derive(Default)]
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

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
    }
}

impl Display for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{BLACK_BISHOP}")
    }
}

impl Debug for BlackBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait BlackBishopExt: BishopExt {}

impl BlackBishopExt for BlackBishop {}
impl BishopExt for BlackBishop {}
impl PieceExt for BlackBishop {}
