use bitboard::Bitboard;
use fen::ForsythEdwardsNotation;
use pieces::{
    bishop::{black::BlackBishop, white::WhiteBishop},
    king::{black::BlackKing, white::WhiteKing},
    knight::{black::BlackKnight, white::WhiteKnight},
    pawn::{black::BlackPawn, white::WhitePawn},
    queen::{black::BlackQueen, white::WhiteQueen},
    rook::{black::BlackRook, white::WhiteRook},
};

pub static BOARD_SIZE: i8 = 8;
pub static NORTH: i8 = BOARD_SIZE;
pub static EAST: i8 = 1;
pub static SOUTH: i8 = -BOARD_SIZE;
pub static WEST: i8 = -EAST;
pub static NORTH_EAST: i8 = NORTH + EAST;
pub static SOUTH_EAST: i8 = SOUTH + EAST;
pub static SOUTH_WEST: i8 = SOUTH + WEST;
pub static NORTH_WEST: i8 = NORTH + WEST;
pub static NORTH_NORTH_EAST: i8 = NORTH + NORTH_EAST;
pub static NORTH_NORTH_WEST: i8 = NORTH + NORTH_WEST;
pub static SOUTH_SOUTH_EAST: i8 = SOUTH + SOUTH_EAST;
pub static SOUTH_SOUTH_WEST: i8 = SOUTH + SOUTH_WEST;
pub static EAST_EAST_NORTH: i8 = EAST + NORTH_EAST;
pub static EAST_EAST_SOUTH: i8 = EAST + SOUTH_EAST;
pub static WEST_WEST_NORTH: i8 = WEST + NORTH_WEST;
pub static WEST_WEST_SOUTH: i8 = WEST + SOUTH_WEST;

#[derive(Debug)]
pub struct Board {
    fen: ForsythEdwardsNotation,
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
