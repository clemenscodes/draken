use super::{Pawn, PawnExt};
use crate::{
    moves::{encoded_move::EncodedMove, reversible::quiet::QuietMove},
    pieces::{Piece, PieceExt, WhiteBishop, WhiteKnight, WhiteQueen, WhiteRook},
    Board, Shift, Verify, FOURTH_RANK,
};
use api::Square;
use bitboard::{Bitboard, BitboardExt};
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

    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Pawn::from(*self).get_attacks(piece, board)
    }
}

impl Verify for WhitePawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        Pawn::from(*self).verify(source, destination, board)
    }
}

impl PawnExt for WhitePawn {
    #[inline(always)]
    fn get_west_attacks(&self, pawns: Bitboard) -> Bitboard {
        Self::west_attack_mask() & Bitboard::shift_north_west(pawns)
    }

    #[inline(always)]
    fn get_east_attacks(&self, pawns: Bitboard) -> Bitboard {
        Self::east_attack_mask() & Bitboard::shift_north_east(pawns)
    }

    #[inline(always)]
    fn get_attacking_pawns(&self, board: Board) -> Bitboard {
        let attacks = self.get_attacks(self.bitboard(), board);
        Bitboard::shift_south_west(attacks) | Bitboard::shift_south_east(attacks)
    }

    #[inline(always)]
    fn get_single_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        Bitboard::shift_north(pawn) & empty_squares
    }

    #[inline(always)]
    fn get_double_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        let double_push_mask = empty_squares & FOURTH_RANK;
        let single_push_targets = self.get_single_push_targets(pawn, empty_squares);
        let double_push_targets = Bitboard::shift_north(single_push_targets);
        double_push_targets & double_push_mask
    }

    #[inline(always)]
    fn get_single_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard {
        self.bitboard() & Bitboard::shift_south(empty_squares)
    }

    #[inline(always)]
    fn get_double_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard {
        let empty_skipped_rank = Bitboard::shift_south(empty_squares & FOURTH_RANK) & empty_squares;
        self.get_single_pushable_pawns(empty_skipped_rank)
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
