pub mod black;
pub mod white;

use super::{Piece, PieceExt};
use crate::{
    moves::{encoded_move::EncodedMove, reversible::quiet::QuietMove},
    Board, Verify, EIGHTH_FILE, EIGHTH_RANK, FIFTH_FILE, FIRST_RANK,
};
use api::{ForsythEdwardsNotationExt, Square};
use bitboard::{Bitboard, BitboardExt};
use black::BlackPawn;
use white::WhitePawn;

#[derive(Debug)]
pub enum Pawn {
    Black(BlackPawn),
    White(WhitePawn),
}

pub trait PawnExt: PieceExt {
    fn get_west_attacks(&self, pawns: Bitboard) -> Bitboard;
    fn get_east_attacks(&self, pawns: Bitboard) -> Bitboard;
    fn get_attacking_pawns(&self, board: Board) -> Bitboard;
    fn get_single_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard;
    fn get_double_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard;
    fn get_single_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard;
    fn get_double_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard;
    fn get_promotion_pieces(&self) -> [Piece; 4];
    fn march(&mut self, source: Square, destination: Square, board: &mut Board) -> Result<(), ()> {
        if self.is_illegal_move(source, destination, *board) {
            eprint!("Illegal pawn move: Can not move from {source} to {destination}");
            return Err(());
        }
        Ok(())
    }
    fn get_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        self.get_single_push_targets(pawn, empty_squares) | self.get_double_push_targets(pawn, empty_squares)
    }
    fn get_targets(&self, pawn: Bitboard, board: Board) -> Bitboard {
        self.get_push_targets(pawn, board.pieces().empty_squares()) | self.get_attacks(pawn, board)
    }
    fn promotion_mask() -> Bitboard {
        FIRST_RANK | EIGHTH_RANK
    }
    fn west_attack_mask() -> Bitboard {
        !EIGHTH_FILE
    }
    fn east_attack_mask() -> Bitboard {
        !FIFTH_FILE
    }
}

impl From<WhitePawn> for Pawn {
    fn from(v: WhitePawn) -> Self {
        Self::White(v)
    }
}

impl From<BlackPawn> for Pawn {
    fn from(v: BlackPawn) -> Self {
        Self::Black(v)
    }
}

impl Verify for Pawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        let pawn = Bitboard::get_single_bit(source.into());
        if !Bitboard::check_bit(self.get_targets(pawn, board), destination.into()) {
            return Err(());
        }
        let encoded_move = EncodedMove::from(QuietMove::new(source, destination));
        Ok(encoded_move.data())
    }
}

impl PieceExt for Pawn {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        match self {
            Pawn::Black(pawn) => pawn.is_illegal_move(source, destination, board),
            Pawn::White(pawn) => pawn.is_illegal_move(source, destination, board),
        }
    }

    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        let enpassant_mask = board.fen().enpassant_mask();
        let west_attacks = self.get_west_attacks(piece);
        let east_attacks = self.get_east_attacks(piece);
        let regular_attacks = west_attacks | east_attacks;
        let enemy_pieces: Bitboard = if board.fen().is_white() {
            board.pieces().black_pieces().into()
        } else {
            board.pieces().white_pieces().into()
        };
        let mut enemy_attacks = regular_attacks & enemy_pieces;
        if regular_attacks.self_overlap(enpassant_mask) {
            enemy_attacks |= enpassant_mask;
        }
        self.remove_friendly_pieces(enemy_attacks, board)
    }
}

impl PawnExt for Pawn {
    fn get_promotion_pieces(&self) -> [Piece; 4] {
        match self {
            Pawn::Black(pawn) => pawn.get_promotion_pieces(),
            Pawn::White(pawn) => pawn.get_promotion_pieces(),
        }
    }

    fn get_west_attacks(&self, pawns: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_west_attacks(pawns),
            Pawn::White(pawn) => pawn.get_west_attacks(pawns),
        }
    }

    fn get_east_attacks(&self, pawns: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_east_attacks(pawns),
            Pawn::White(pawn) => pawn.get_east_attacks(pawns),
        }
    }

    fn get_attacking_pawns(&self, board: Board) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_attacking_pawns(board),
            Pawn::White(pawn) => pawn.get_attacking_pawns(board),
        }
    }

    fn get_single_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(p) => p.get_single_push_targets(pawn, empty_squares),
            Pawn::White(p) => p.get_single_push_targets(pawn, empty_squares),
        }
    }

    fn get_double_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(p) => p.get_double_push_targets(pawn, empty_squares),
            Pawn::White(p) => p.get_double_push_targets(pawn, empty_squares),
        }
    }

    fn get_single_pushable_pawns(&self, empty_squres: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_single_pushable_pawns(empty_squres),
            Pawn::White(pawn) => pawn.get_single_pushable_pawns(empty_squres),
        }
    }

    fn get_double_pushable_pawns(&self, empty_squres: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_double_pushable_pawns(empty_squres),
            Pawn::White(pawn) => pawn.get_double_pushable_pawns(empty_squres),
        }
    }
}
