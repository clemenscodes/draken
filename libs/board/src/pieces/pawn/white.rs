use super::PawnExt;
use crate::{
    moves::{encoded_move::EncodedMove, reversible::quiet::QuietMove},
    pieces::{PieceExt, Verify},
    Board,
};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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

    pub fn bitboard_mut(&mut self) -> &mut Bitboard {
        &mut self.bitboard
    }

    pub const fn symbol() -> char {
        'P'
    }

    pub const fn utf_symbol() -> char {
        '♙'
    }
}

impl From<Bitboard> for WhitePawn {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhitePawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhitePawn::symbol())
    }
}

impl Debug for WhitePawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PawnExt for WhitePawn {}
impl PieceExt for WhitePawn {}

impl Verify for WhitePawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        println!("Verifying white pawn move from {source} to {destination}");
        let encoded_move = EncodedMove::from(QuietMove::new(source, destination));
        Ok(encoded_move.data())
    }
}
