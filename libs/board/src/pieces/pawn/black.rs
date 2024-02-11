use super::PawnExt;
use crate::{
    pieces::{BlackBishop, BlackKnight, BlackQueen, BlackRook, Piece, PieceExt},
    Board, Verify,
};
use api::Square;
use bitboard::Bitboard;
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackPawn {
    bitboard: Bitboard,
}

impl BlackPawn {
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
        'p'
    }

    pub const fn utf_symbol() -> char {
        '♟'
    }
}

impl From<Bitboard> for BlackPawn {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackPawn::symbol())
    }
}

impl Debug for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for BlackPawn {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        todo!()
    }
}

impl Verify for BlackPawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!();
    }
}

impl PawnExt for BlackPawn {
    fn get_west_attacks(&self) -> Bitboard {
        todo!()
    }

    fn get_east_attacks(&self) -> Bitboard {
        todo!()
    }

    fn get_attacking_pawns(&self) -> Bitboard {
        todo!()
    }

    fn get_single_push_targets(&self) -> Bitboard {
        todo!()
    }

    fn get_double_push_targets(&self) -> Bitboard {
        todo!()
    }

    fn get_single_pushable_pawns(&self) -> Bitboard {
        todo!()
    }

    fn get_double_pushable_pawns(&self) -> Bitboard {
        todo!()
    }

    fn get_promotion_pieces(&self) -> [Piece; 4] {
        [
            Piece::from(BlackBishop::default()),
            Piece::from(BlackRook::default()),
            Piece::from(BlackKnight::default()),
            Piece::from(BlackQueen::default()),
        ]
    }
}
