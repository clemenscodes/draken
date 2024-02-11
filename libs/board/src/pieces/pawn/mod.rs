pub mod black;
pub mod white;

use super::{Piece, PieceExt};
use crate::{Board, Verify, EIGHTH_RANK, FIRST_RANK};
use api::Square;
use bitboard::Bitboard;
use black::BlackPawn;
use white::WhitePawn;

#[derive(Debug)]
pub enum Pawn {
    Black(BlackPawn),
    White(WhitePawn),
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

pub trait PawnExt: PieceExt {
    fn get_west_attacks(&self) -> Bitboard;
    fn get_east_attacks(&self) -> Bitboard;
    fn get_attacking_pawns(&self) -> Bitboard;
    fn get_single_push_targets(&self) -> Bitboard;
    fn get_double_push_targets(&self) -> Bitboard;
    fn get_single_pushable_pawns(&self) -> Bitboard;
    fn get_double_pushable_pawns(&self) -> Bitboard;
    fn get_promotion_pieces(&self) -> [Piece; 4];
    fn promotion_mask() -> Bitboard {
        FIRST_RANK | EIGHTH_RANK
    }
}

impl PieceExt for Pawn {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        match self {
            Pawn::Black(pawn) => pawn.is_illegal_move(source, destination, board),
            Pawn::White(pawn) => pawn.is_illegal_move(source, destination, board),
        }
    }
}

impl Verify for Pawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        match self {
            Pawn::Black(pawn) => pawn.verify(source, destination, board),
            Pawn::White(pawn) => pawn.verify(source, destination, board),
        }
    }
}

impl PawnExt for Pawn {
    fn get_west_attacks(&self) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_west_attacks(),
            Pawn::White(pawn) => pawn.get_west_attacks(),
        }
    }

    fn get_east_attacks(&self) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_east_attacks(),
            Pawn::White(pawn) => pawn.get_east_attacks(),
        }
    }

    fn get_attacking_pawns(&self) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_attacking_pawns(),
            Pawn::White(pawn) => pawn.get_attacking_pawns(),
        }
    }

    fn get_single_push_targets(&self) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_single_push_targets(),
            Pawn::White(pawn) => pawn.get_single_push_targets(),
        }
    }

    fn get_double_push_targets(&self) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_double_push_targets(),
            Pawn::White(pawn) => pawn.get_double_push_targets(),
        }
    }

    fn get_single_pushable_pawns(&self) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_single_pushable_pawns(),
            Pawn::White(pawn) => pawn.get_single_pushable_pawns(),
        }
    }

    fn get_double_pushable_pawns(&self) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_double_pushable_pawns(),
            Pawn::White(pawn) => pawn.get_double_pushable_pawns(),
        }
    }

    fn get_promotion_pieces(&self) -> [Piece; 4] {
        match self {
            Pawn::Black(pawn) => pawn.get_promotion_pieces(),
            Pawn::White(pawn) => pawn.get_promotion_pieces(),
        }
    }
}
