use super::{Bishop, BishopExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteBishop {
    bitboard: Bitboard,
}

impl WhiteBishop {
    pub const SYMBOL: char = 'B';
    pub const UTF_SYMBOL: char = '♗';

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

impl From<Bitboard> for WhiteBishop {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteBishop::SYMBOL)
    }
}

impl Debug for WhiteBishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for WhiteBishop {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        Bishop::from(*self).is_illegal_move(source, destination, board)
    }

    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Bishop::from(*self).get_attacks(piece, board)
    }
}

impl Verify for WhiteBishop {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Bishop::from(*self).verify(source, destination, promotion, board)
    }
}

impl BishopExt for WhiteBishop {}
