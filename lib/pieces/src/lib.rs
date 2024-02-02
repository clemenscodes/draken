#![feature(lazy_cell)]
mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use bishop::{black::*, white::*, Bishop};
use bitboard::{Bitboard, BitboardExt};
use king::{black::*, white::*, King};
use knight::{black::*, white::*, Knight};
use pawn::{black::*, white::*, Pawn};
use queen::{black::*, white::*, Queen};
use rook::{black::*, white::*, Rook};
use std::{collections::HashMap, sync::LazyLock};

pub const NUM_PIECES: usize = 12;

pub const EMPTY_SYMBOL: char = ' ';

pub const PIECE_SYMBOLS: [char; NUM_PIECES] = [
    BLACK_ROOK,
    BLACK_KNIGHT,
    BLACK_BISHOP,
    BLACK_QUEEN,
    BLACK_KING,
    BLACK_PAWN,
    WHITE_ROOK,
    WHITE_KNIGHT,
    WHITE_BISHOP,
    WHITE_QUEEN,
    WHITE_KING,
    WHITE_PAWN,
];

pub const PIECE_BYTES: [u8; NUM_PIECES] = [
    BLACK_ROOK as u8,
    BLACK_KNIGHT as u8,
    BLACK_BISHOP as u8,
    BLACK_QUEEN as u8,
    BLACK_KING as u8,
    BLACK_PAWN as u8,
    WHITE_ROOK as u8,
    WHITE_KNIGHT as u8,
    WHITE_BISHOP as u8,
    WHITE_QUEEN as u8,
    WHITE_KING as u8,
    WHITE_PAWN as u8,
];

pub const PIECE_INDEX_LOOKUP_MAP: LazyLock<HashMap<char, usize>> = LazyLock::new(|| {
    let mut piece_lookup: HashMap<char, usize> = HashMap::new();
    for (i, &piece) in PIECE_SYMBOLS.iter().enumerate() {
        piece_lookup.insert(piece, i);
    }
    piece_lookup
});

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
            invalid => {
                eprintln!("Invalid character {invalid}");
                return Err(Self::Error::Invalid);
            }
        };
        Ok(piece)
    }
}

impl From<Pawn> for Piece {
    fn from(v: Pawn) -> Self {
        Self::Pawn(v)
    }
}

impl From<King> for Piece {
    fn from(v: King) -> Self {
        Self::King(v)
    }
}

impl From<Queen> for Piece {
    fn from(v: Queen) -> Self {
        Self::Queen(v)
    }
}

impl From<Bishop> for Piece {
    fn from(v: Bishop) -> Self {
        Self::Bishop(v)
    }
}

impl From<Knight> for Piece {
    fn from(v: Knight) -> Self {
        Self::Knight(v)
    }
}

impl From<Rook> for Piece {
    fn from(v: Rook) -> Self {
        Self::Rook(v)
    }
}

pub trait PieceExt {}

impl PieceExt for Piece {}

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

#[derive(Default, Debug)]
pub struct Pieces {
    white_king: WhiteKing,
    black_king: BlackKing,
    white_bishop: WhiteBishop,
    black_bishop: BlackBishop,
    white_queen: WhiteQueen,
    black_queen: BlackQueen,
    white_rook: WhiteRook,
    black_rook: BlackRook,
    white_knight: WhiteKnight,
    black_knight: BlackKnight,
    white_pawn: WhitePawn,
    black_pawn: BlackPawn,
}

impl Pieces {
    pub fn new(
        white_king: WhiteKing,
        black_king: BlackKing,
        white_bishop: WhiteBishop,
        black_bishop: BlackBishop,
        white_queen: WhiteQueen,
        black_queen: BlackQueen,
        white_rook: WhiteRook,
        black_rook: BlackRook,
        white_knight: WhiteKnight,
        black_knight: BlackKnight,
        white_pawn: WhitePawn,
        black_pawn: BlackPawn,
    ) -> Self {
        Self {
            white_king,
            black_king,
            white_bishop,
            black_bishop,
            white_queen,
            black_queen,
            white_rook,
            black_rook,
            white_knight,
            black_knight,
            white_pawn,
            black_pawn,
        }
    }

    pub fn white_king(&self) -> &WhiteKing {
        &self.white_king
    }

    pub fn black_king(&self) -> &BlackKing {
        &self.black_king
    }

    pub fn white_bishop(&self) -> &WhiteBishop {
        &self.white_bishop
    }

    pub fn black_bishop(&self) -> &BlackBishop {
        &self.black_bishop
    }

    pub fn white_queen(&self) -> &WhiteQueen {
        &self.white_queen
    }

    pub fn black_queen(&self) -> &BlackQueen {
        &self.black_queen
    }

    pub fn white_rook(&self) -> &WhiteRook {
        &self.white_rook
    }

    pub fn black_rook(&self) -> &BlackRook {
        &self.black_rook
    }

    pub fn white_knight(&self) -> &WhiteKnight {
        &self.white_knight
    }

    pub fn black_knight(&self) -> &BlackKnight {
        &self.black_knight
    }

    pub fn white_pawn(&self) -> &WhitePawn {
        &self.white_pawn
    }

    pub fn black_pawn(&self) -> &BlackPawn {
        &self.black_pawn
    }

    pub fn white_king_mut(&mut self) -> &mut WhiteKing {
        &mut self.white_king
    }

    pub fn black_king_mut(&mut self) -> &mut BlackKing {
        &mut self.black_king
    }

    pub fn white_bishop_mut(&mut self) -> &mut WhiteBishop {
        &mut self.white_bishop
    }

    pub fn black_bishop_mut(&mut self) -> &mut BlackBishop {
        &mut self.black_bishop
    }

    pub fn white_queen_mut(&mut self) -> &mut WhiteQueen {
        &mut self.white_queen
    }

    pub fn black_queen_mut(&mut self) -> &mut BlackQueen {
        &mut self.black_queen
    }

    pub fn white_rook_mut(&mut self) -> &mut WhiteRook {
        &mut self.white_rook
    }

    pub fn black_rook_mut(&mut self) -> &mut BlackRook {
        &mut self.black_rook
    }

    pub fn white_knight_mut(&mut self) -> &mut WhiteKnight {
        &mut self.white_knight
    }

    pub fn black_knight_mut(&mut self) -> &mut BlackKnight {
        &mut self.black_knight
    }

    pub fn white_pawn_mut(&mut self) -> &mut WhitePawn {
        &mut self.white_pawn
    }

    pub fn black_pawn_mut(&mut self) -> &mut BlackPawn {
        &mut self.black_pawn
    }

    pub fn merge_piece(&mut self, mut piece: Piece) {
        let board = piece.get_board().clone();
        match piece {
            Piece::Rook(rook) => match rook {
                Rook::Black(_) => self.black_rook_mut().bitboard_mut().self_merge(board),
                Rook::White(_) => self.white_rook_mut().bitboard_mut().self_merge(board),
            },
            Piece::Knight(knight) => match knight {
                Knight::Black(_) => self.black_knight_mut().bitboard_mut().self_merge(board),
                Knight::White(_) => self.white_knight_mut().bitboard_mut().self_merge(board),
            },
            Piece::Bishop(bishop) => match bishop {
                Bishop::Black(_) => self.black_bishop_mut().bitboard_mut().self_merge(board),
                Bishop::White(_) => self.white_bishop_mut().bitboard_mut().self_merge(board),
            },
            Piece::Queen(queen) => match queen {
                Queen::Black(_) => self.black_queen_mut().bitboard_mut().self_merge(board),
                Queen::White(_) => self.white_queen_mut().bitboard_mut().self_merge(board),
            },
            Piece::King(king) => match king {
                King::Black(_) => self.black_king_mut().bitboard_mut().self_merge(board),
                King::White(_) => self.white_king_mut().bitboard_mut().self_merge(board),
            },
            Piece::Pawn(pawn) => match pawn {
                Pawn::Black(_) => self.black_pawn_mut().bitboard_mut().self_merge(board),
                Pawn::White(_) => self.white_pawn_mut().bitboard_mut().self_merge(board),
            },
        };
    }
}

pub trait PiecesExt {}

impl PiecesExt for Pieces {}
