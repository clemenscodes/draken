use super::{WhiteBishop, WhiteKing, WhiteKnight, WhitePawn, WhiteQueen, WhiteRook};
use bitboard::{Bitboard, BitboardExt};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhitePieces {
    rook: WhiteRook,
    knight: WhiteKnight,
    bishop: WhiteBishop,
    queen: WhiteQueen,
    king: WhiteKing,
    pawn: WhitePawn,
}

impl WhitePieces {
    pub fn rook(&self) -> WhiteRook {
        self.rook
    }

    pub fn rook_mut(&mut self) -> &mut WhiteRook {
        &mut self.rook
    }

    pub fn knight(&self) -> WhiteKnight {
        self.knight
    }

    pub fn knight_mut(&mut self) -> &mut WhiteKnight {
        &mut self.knight
    }

    pub fn bishop(&self) -> WhiteBishop {
        self.bishop
    }

    pub fn bishop_mut(&mut self) -> &mut WhiteBishop {
        &mut self.bishop
    }

    pub fn queen(&self) -> WhiteQueen {
        self.queen
    }

    pub fn queen_mut(&mut self) -> &mut WhiteQueen {
        &mut self.queen
    }

    pub fn king(&self) -> WhiteKing {
        self.king
    }

    pub fn king_mut(&mut self) -> &mut WhiteKing {
        &mut self.king
    }

    pub fn pawn(&self) -> WhitePawn {
        self.pawn
    }

    pub fn pawn_mut(&mut self) -> &mut WhitePawn {
        &mut self.pawn
    }
}

impl Into<Bitboard> for WhitePieces {
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
