#![feature(lazy_cell)]
pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

use std::{collections::HashMap, sync::LazyLock};

use bishop::black::*;
use bishop::white::*;
use bishop::Bishop;
use bitboard::Bitboard;
use king::black::*;
use king::white::*;
use king::King;
use knight::black::*;
use knight::white::*;
use knight::Knight;
use pawn::black::*;
use pawn::white::*;
use pawn::Pawn;
use queen::black::*;
use queen::white::*;
use queen::Queen;
use rook::black::*;
use rook::white::*;
use rook::Rook;

pub const NUM_PIECES: usize = 12;

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

pub static PIECE_INDEX_LOOKUP_MAP: LazyLock<HashMap<char, usize>> = LazyLock::new(|| {
    let mut piece_lookup: HashMap<char, usize> = HashMap::new();
    for (i, &piece) in PIECE_SYMBOLS.iter().enumerate() {
        piece_lookup.insert(piece, i);
    }
    piece_lookup
});

#[derive(Debug)]
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
    white_pieces: Bitboard,
    black_pieces: Bitboard,
    occupied_squares: Bitboard,
    empty_squares: Bitboard,
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
        white_pieces: Bitboard,
        black_pieces: Bitboard,
        occupied_squares: Bitboard,
        empty_squares: Bitboard,
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
            white_pieces,
            black_pieces,
            occupied_squares,
            empty_squares,
        }
    }
}

#[derive(Debug)]
pub enum Piece {
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
    Pawn(Pawn),
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
