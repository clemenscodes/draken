#![feature(lazy_cell)]
pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use std::{collections::HashMap, sync::LazyLock};

use bishop::black::BLACK_BISHOP;
use bishop::white::WHITE_BISHOP;
use bishop::Bishop;
use king::black::BLACK_KING;
use king::white::WHITE_KING;
use king::King;
use knight::black::BLACK_KNIGHT;
use knight::white::WHITE_KNIGHT;
use knight::Knight;
use pawn::black::BLACK_PAWN;
use pawn::white::WHITE_PAWN;
use pawn::Pawn;
use queen::black::BLACK_QUEEN;
use queen::white::WHITE_QUEEN;
use queen::Queen;
use rook::black::BLACK_ROOK;
use rook::white::WHITE_ROOK;
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
