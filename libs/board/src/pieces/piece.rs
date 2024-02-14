use super::*;
use crate::{Board, Verify};
use api::{ForsythEdwardsNotationExt, Square};
use bitboard::Bitboard;
use std::error::Error;

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
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard;
    fn remove_friendly_pieces(&self, piece: Bitboard, board: Board) -> Bitboard {
        let friendly_pieces: Bitboard = if board.fen().is_white() {
            board.pieces().white_pieces().into()
        } else {
            board.pieces().black_pieces().into()
        };
        let not_friendly_pieces = !friendly_pieces;
        piece & not_friendly_pieces
    }
}

impl TryFrom<char> for Piece {
    type Error = PieceError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let piece: Piece = match value {
            piece if piece == PIECE_SYMBOLS[0] => Piece::from(BlackRook::default()),
            piece if piece == PIECE_SYMBOLS[1] => Piece::from(BlackKnight::default()),
            piece if piece == PIECE_SYMBOLS[2] => Piece::from(BlackBishop::default()),
            piece if piece == PIECE_SYMBOLS[3] => Piece::from(BlackQueen::default()),
            piece if piece == PIECE_SYMBOLS[4] => Piece::from(BlackKing::default()),
            piece if piece == PIECE_SYMBOLS[5] => Piece::from(BlackPawn::default()),
            piece if piece == PIECE_SYMBOLS[6] => Piece::from(WhiteRook::default()),
            piece if piece == PIECE_SYMBOLS[7] => Piece::from(WhiteKnight::default()),
            piece if piece == PIECE_SYMBOLS[8] => Piece::from(WhiteBishop::default()),
            piece if piece == PIECE_SYMBOLS[9] => Piece::from(WhiteQueen::default()),
            piece if piece == PIECE_SYMBOLS[10] => Piece::from(WhiteKing::default()),
            piece if piece == PIECE_SYMBOLS[11] => Piece::from(WhitePawn::default()),
            _ => return Err(Self::Error::Invalid),
        };
        Ok(piece)
    }
}

impl From<Pawn> for Piece {
    fn from(v: Pawn) -> Self {
        Self::Pawn(v)
    }
}

impl From<WhitePawn> for Piece {
    fn from(v: WhitePawn) -> Self {
        Self::Pawn(Pawn::from(v))
    }
}

impl From<BlackPawn> for Piece {
    fn from(v: BlackPawn) -> Self {
        Self::Pawn(Pawn::from(v))
    }
}

impl From<King> for Piece {
    fn from(v: King) -> Self {
        Self::King(v)
    }
}

impl From<WhiteKing> for Piece {
    fn from(v: WhiteKing) -> Self {
        Self::King(King::from(v))
    }
}

impl From<BlackKing> for Piece {
    fn from(v: BlackKing) -> Self {
        Self::King(King::from(v))
    }
}

impl From<Queen> for Piece {
    fn from(v: Queen) -> Self {
        Self::Queen(v)
    }
}

impl From<WhiteQueen> for Piece {
    fn from(v: WhiteQueen) -> Self {
        Self::Queen(Queen::from(v))
    }
}

impl From<BlackQueen> for Piece {
    fn from(v: BlackQueen) -> Self {
        Self::Queen(Queen::from(v))
    }
}

impl From<Bishop> for Piece {
    fn from(v: Bishop) -> Self {
        Self::Bishop(v)
    }
}

impl From<BlackBishop> for Piece {
    fn from(v: BlackBishop) -> Self {
        Self::Bishop(Bishop::from(v))
    }
}

impl From<WhiteBishop> for Piece {
    fn from(v: WhiteBishop) -> Self {
        Self::Bishop(Bishop::from(v))
    }
}

impl From<Knight> for Piece {
    fn from(v: Knight) -> Self {
        Self::Knight(v)
    }
}

impl From<WhiteKnight> for Piece {
    fn from(v: WhiteKnight) -> Self {
        Self::Knight(Knight::from(v))
    }
}

impl From<BlackKnight> for Piece {
    fn from(v: BlackKnight) -> Self {
        Self::Knight(Knight::from(v))
    }
}

impl From<Rook> for Piece {
    fn from(v: Rook) -> Self {
        Self::Rook(v)
    }
}

impl From<WhiteRook> for Piece {
    fn from(v: WhiteRook) -> Self {
        Self::Rook(Rook::from(v))
    }
}

impl From<BlackRook> for Piece {
    fn from(v: BlackRook) -> Self {
        Self::Rook(Rook::from(v))
    }
}

impl Piece {
    pub fn get_board_mut(&mut self) -> &mut Bitboard {
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

    pub fn get_board(&self) -> Bitboard {
        match self {
            Piece::Rook(rook) => match rook {
                Rook::Black(rook) => rook.bitboard(),
                Rook::White(rook) => rook.bitboard(),
            },
            Piece::Knight(knight) => match knight {
                Knight::Black(knight) => knight.bitboard(),
                Knight::White(knight) => knight.bitboard(),
            },
            Piece::Bishop(bishop) => match bishop {
                Bishop::Black(bishop) => bishop.bitboard(),
                Bishop::White(bishop) => bishop.bitboard(),
            },
            Piece::Queen(queen) => match queen {
                Queen::Black(queen) => queen.bitboard(),
                Queen::White(queen) => queen.bitboard(),
            },
            Piece::King(king) => match king {
                King::Black(king) => king.bitboard(),
                King::White(king) => king.bitboard(),
            },
            Piece::Pawn(pawn) => match pawn {
                Pawn::Black(pawn) => pawn.bitboard(),
                Pawn::White(pawn) => pawn.bitboard(),
            },
        }
    }

    pub fn set_on_square(&mut self, rank: u8, file: u8) {
        let board = Bitboard::try_from((rank as usize, file as usize)).unwrap();
        self.get_board_mut().self_merge(board);
    }

    /// Returns `true` if the piece is [`King`].
    ///
    /// [`King`]: Piece::King
    #[must_use]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(..))
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

    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        match self {
            Piece::Rook(rook) => rook.get_attacks(piece, board),
            Piece::Knight(knight) => knight.get_attacks(piece, board),
            Piece::Bishop(bishop) => bishop.get_attacks(piece, board),
            Piece::Queen(queen) => queen.get_attacks(piece, board),
            Piece::King(king) => king.get_attacks(piece, board),
            Piece::Pawn(pawn) => pawn.get_attacks(piece, board),
        }
    }
}

impl Verify for Piece {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        match self {
            Piece::Rook(rook) => rook.verify(source, destination, promotion, board),
            Piece::Knight(knight) => knight.verify(source, destination, promotion, board),
            Piece::Bishop(bishop) => bishop.verify(source, destination, promotion, board),
            Piece::Queen(queen) => queen.verify(source, destination, promotion, board),
            Piece::King(king) => king.verify(source, destination, promotion, board),
            Piece::Pawn(pawn) => pawn.verify(source, destination, promotion, board),
        }
    }
}
