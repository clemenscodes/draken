use super::{Queen, QueenExt};
use crate::{pieces::PieceExt, Board, Verify};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WhiteQueen {
    bitboard: Bitboard,
}

impl WhiteQueen {
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
        'Q'
    }

    pub const fn utf_symbol() -> char {
        '♕'
    }
}

impl From<Bitboard> for WhiteQueen {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for WhiteQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WhiteQueen::symbol())
    }
}

impl Debug for WhiteQueen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueenExt for WhiteQueen {}
impl PieceExt for WhiteQueen {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        Queen::from(*self).is_illegal_move(source, destination, board)
    }
}

impl Verify for WhiteQueen {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Queen::from(*self).verify(source, destination, board)
    }
}
