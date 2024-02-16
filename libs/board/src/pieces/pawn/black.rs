use super::{Pawn, PawnExt};
use crate::{
    pieces::{BlackBishop, BlackKnight, BlackQueen, BlackRook, Piece, PieceExt},
    Board, Shift, Verify, FIFTH_RANK,
};
use api::Square;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct BlackPawn {
    bitboard: Bitboard,
}

impl BlackPawn {
    pub const SYMBOL: char = 'p';
    pub const UTF_SYMBOL: char = '♟';

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

impl From<Bitboard> for BlackPawn {
    fn from(value: Bitboard) -> Self {
        Self::new(value)
    }
}

impl Display for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BlackPawn::SYMBOL)
    }
}

impl Debug for BlackPawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl PieceExt for BlackPawn {
    #[inline(always)]
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        Pawn::from(*self).get_attacks(piece, board)
    }
}

impl Verify for BlackPawn {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        Pawn::from(*self).verify(source, destination, promotion, board)
    }
}

impl PawnExt for BlackPawn {
    #[inline(always)]
    fn get_west_attacks(&self, pawns: Bitboard) -> Bitboard {
        Self::west_attack_mask() & Bitboard::shift_south_west(pawns)
    }

    #[inline(always)]
    fn get_east_attacks(&self, pawns: Bitboard) -> Bitboard {
        Self::east_attack_mask() & Bitboard::shift_south_east(pawns)
    }

    #[inline(always)]
    fn get_attacking_pawns(&self, board: Board) -> Bitboard {
        let attacks = self.get_attacks(self.bitboard(), board);
        Bitboard::shift_north_west(attacks) | Bitboard::shift_north_east(attacks)
    }

    #[inline(always)]
    fn get_single_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        Bitboard::shift_south(pawn) & empty_squares
    }

    #[inline(always)]
    fn get_double_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        let double_push_mask = empty_squares & FIFTH_RANK;
        let single_push_targets = self.get_single_push_targets(pawn, empty_squares);
        let double_push_targets = Bitboard::shift_south(single_push_targets);
        double_push_targets & double_push_mask
    }

    #[inline(always)]
    fn get_single_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard {
        self.bitboard() & Bitboard::shift_north(empty_squares)
    }

    #[inline(always)]
    fn get_double_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard {
        let empty_skipped_rank = Bitboard::shift_north(empty_squares & FIFTH_RANK) & empty_squares;
        self.get_single_pushable_pawns(empty_skipped_rank)
    }

    #[inline(always)]
    fn get_promotion_pieces(&self) -> [Piece; 4] {
        [
            Piece::from(BlackBishop::default()),
            Piece::from(BlackRook::default()),
            Piece::from(BlackKnight::default()),
            Piece::from(BlackQueen::default()),
        ]
    }
}
