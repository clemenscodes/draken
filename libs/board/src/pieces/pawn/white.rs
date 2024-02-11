use super::{Pawn, PawnExt};
use crate::{
    moves::{encoded_move::EncodedMove, reversible::quiet::QuietMove},
    pieces::{Piece, PieceExt, WhiteBishop, WhiteKnight, WhiteQueen, WhiteRook},
    Board, Verify,
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

impl PieceExt for WhitePawn {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        todo!()
    }

    fn get_attacks(&self, piece: Bitboard, board: &mut Board) -> Bitboard {
        Pawn::from(*self).get_attacks(piece, board)
    }
}

impl Verify for WhitePawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        println!("Verifying white pawn move from {source} to {destination}");
        let encoded_move = EncodedMove::from(QuietMove::new(source, destination));
        Ok(encoded_move.data())
    }
}

impl PawnExt for WhitePawn {
    fn get_west_attacks(&self, pawns: Bitboard) -> Bitboard {
        todo!()
    }

    fn get_east_attacks(&self, pawns: Bitboard) -> Bitboard {
        todo!()
    }

    fn get_attacking_pawns(&self, board: &mut Board) -> Bitboard {
        todo!()
    }

    fn get_single_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        todo!()
    }

    fn get_double_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        todo!()
    }

    fn get_single_pushable_pawns(&self, empty_squres: Bitboard) -> Bitboard {
        todo!()
    }

    fn get_double_pushable_pawns(&self, empty_squres: Bitboard) -> Bitboard {
        todo!()
    }

    fn get_promotion_pieces(&self) -> [Piece; 4] {
        [
            Piece::from(WhiteBishop::default()),
            Piece::from(WhiteRook::default()),
            Piece::from(WhiteKnight::default()),
            Piece::from(WhiteQueen::default()),
        ]
    }
}
