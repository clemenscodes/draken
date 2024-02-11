use super::{BlackBishop, BlackKing, BlackKnight, BlackPawn, BlackQueen, BlackRook};
use bitboard::{Bitboard, BitboardExt};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlackPieces {
    rook: BlackRook,
    knight: BlackKnight,
    bishop: BlackBishop,
    queen: BlackQueen,
    king: BlackKing,
    pawn: BlackPawn,
}

impl BlackPieces {
    pub fn rook(&self) -> BlackRook {
        self.rook
    }

    pub fn rook_mut(&mut self) -> &mut BlackRook {
        &mut self.rook
    }

    pub fn knight(&self) -> BlackKnight {
        self.knight
    }

    pub fn knight_mut(&mut self) -> &mut BlackKnight {
        &mut self.knight
    }

    pub fn bishop(&self) -> BlackBishop {
        self.bishop
    }

    pub fn bishop_mut(&mut self) -> &mut BlackBishop {
        &mut self.bishop
    }

    pub fn queen(&self) -> BlackQueen {
        self.queen
    }

    pub fn queen_mut(&mut self) -> &mut BlackQueen {
        &mut self.queen
    }

    pub fn king(&self) -> BlackKing {
        self.king
    }

    pub fn king_mut(&mut self) -> &mut BlackKing {
        &mut self.king
    }

    pub fn pawn(&self) -> BlackPawn {
        self.pawn
    }

    pub fn pawn_mut(&mut self) -> &mut BlackPawn {
        &mut self.pawn
    }
}

impl Into<Bitboard> for BlackPieces {
    fn into(self) -> Bitboard {
        Bitboard::merge_many(vec![
            self.rook.bitboard(),
            self.knight.bitboard(),
            self.bishop.bitboard(),
            self.queen.bitboard(),
            self.king.bitboard(),
            self.pawn.bitboard(),
        ])
    }
}
