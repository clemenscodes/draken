use super::*;
use crate::Board;
use api::Square;
use bitboard::Bitboard;

#[derive(Debug)]
pub enum Piece {
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
    Pawn(Pawn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceError {
    Invalid,
}

pub trait PieceExt: Verify {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool;
}

impl TryFrom<char> for Piece {
    type Error = PieceError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let piece: Piece = match value {
            piece if piece == PIECE_SYMBOLS[0] => Piece::Rook(Rook::Black(BlackRook::default())),
            piece if piece == PIECE_SYMBOLS[1] => Piece::Knight(Knight::Black(BlackKnight::default())),
            piece if piece == PIECE_SYMBOLS[2] => Piece::Bishop(Bishop::Black(BlackBishop::default())),
            piece if piece == PIECE_SYMBOLS[3] => Piece::Queen(Queen::Black(BlackQueen::default())),
            piece if piece == PIECE_SYMBOLS[4] => Piece::King(King::Black(BlackKing::default())),
            piece if piece == PIECE_SYMBOLS[5] => Piece::Pawn(Pawn::Black(BlackPawn::default())),
            piece if piece == PIECE_SYMBOLS[6] => Piece::Rook(Rook::White(WhiteRook::default())),
            piece if piece == PIECE_SYMBOLS[7] => Piece::Knight(Knight::White(WhiteKnight::default())),
            piece if piece == PIECE_SYMBOLS[8] => Piece::Bishop(Bishop::White(WhiteBishop::default())),
            piece if piece == PIECE_SYMBOLS[9] => Piece::Queen(Queen::White(WhiteQueen::default())),
            piece if piece == PIECE_SYMBOLS[10] => Piece::King(King::White(WhiteKing::default())),
            piece if piece == PIECE_SYMBOLS[11] => Piece::Pawn(Pawn::White(WhitePawn::default())),
            _ => return Err(Self::Error::Invalid),
        };
        Ok(piece)
    }
}

impl Piece {
    pub fn get_board(&mut self) -> &mut Bitboard {
        match self {
            Piece::Rook(rook) => match rook {
                Rook::Black(rook) => rook.bitboard_mut(),
                Rook::White(rook) => rook.bitboard_mut(),
            },
            Piece::Knight(knight) => match knight {
                Knight::Black(knight) => knight.bitboard_mut(),
                Knight::White(knight) => knight.bitboard_mut(),
            },
            Piece::Bishop(bishop) => match bishop {
                Bishop::Black(bishop) => bishop.bitboard_mut(),
                Bishop::White(bishop) => bishop.bitboard_mut(),
            },
            Piece::Queen(queen) => match queen {
                Queen::Black(queen) => queen.bitboard_mut(),
                Queen::White(queen) => queen.bitboard_mut(),
            },
            Piece::King(king) => match king {
                King::Black(king) => king.bitboard_mut(),
                King::White(king) => king.bitboard_mut(),
            },
            Piece::Pawn(pawn) => match pawn {
                Pawn::Black(pawn) => pawn.bitboard_mut(),
                Pawn::White(pawn) => pawn.bitboard_mut(),
            },
        }
    }

    pub fn set_on_square(&mut self, rank: u8, file: u8) {
        let board = Bitboard::try_from((rank as usize, file as usize)).unwrap();
        self.get_board().self_merge(board);
    }
}

impl PieceExt for Piece {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        match self {
            Piece::Rook(rook) => rook.is_illegal_move(source, destination, board),
            Piece::Knight(knight) => knight.is_illegal_move(source, destination, board),
            Piece::Bishop(bishop) => bishop.is_illegal_move(source, destination, board),
            Piece::Queen(queen) => queen.is_illegal_move(source, destination, board),
            Piece::King(king) => king.is_illegal_move(source, destination, board),
            Piece::Pawn(pawn) => pawn.is_illegal_move(source, destination, board),
        }
    }
}

impl Verify for Piece {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        match self {
            Piece::Rook(rook) => rook.verify(source, destination, board),
            Piece::Knight(knight) => knight.verify(source, destination, board),
            Piece::Bishop(bishop) => bishop.verify(source, destination, board),
            Piece::Queen(queen) => queen.verify(source, destination, board),
            Piece::King(king) => king.verify(source, destination, board),
            Piece::Pawn(pawn) => pawn.verify(source, destination, board),
        }
    }
}
