pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

pub const NUM_PIECES: usize = 12;

pub const PIECES: [char; NUM_PIECES] = [
    rook::black::BLACK_ROOK,
    knight::black::BLACK_KNIGHT,
    bishop::black::BLACK_BISHOP,
    queen::black::BLACK_QUEEN,
    king::black::BLACK_KING,
    pawn::black::BLACK_PAWN,
    rook::white::WHITE_ROOK,
    knight::white::WHITE_KNIGHT,
    bishop::white::WHITE_BISHOP,
    queen::white::WHITE_QUEEN,
    king::white::WHITE_KING,
    pawn::white::WHITE_PAWN,
];

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
