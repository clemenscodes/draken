#![feature(variant_count)]
use bitboard::Bitboard;
use fen::ForsythEdwardsNotation;
use pieces::{Piece, Pieces};
use std::mem::variant_count;

pub const BOARD_SIZE: i8 = 8;
pub const NORTH: i8 = BOARD_SIZE;
pub const EAST: i8 = 1;
pub const SOUTH: i8 = -BOARD_SIZE;
pub const WEST: i8 = -EAST;
pub const NORTH_EAST: i8 = NORTH + EAST;
pub const SOUTH_EAST: i8 = SOUTH + EAST;
pub const SOUTH_WEST: i8 = SOUTH + WEST;
pub const NORTH_WEST: i8 = NORTH + WEST;
pub const NORTH_NORTH_EAST: i8 = NORTH + NORTH_EAST;
pub const NORTH_NORTH_WEST: i8 = NORTH + NORTH_WEST;
pub const SOUTH_SOUTH_EAST: i8 = SOUTH + SOUTH_EAST;
pub const SOUTH_SOUTH_WEST: i8 = SOUTH + SOUTH_WEST;
pub const EAST_EAST_NORTH: i8 = EAST + NORTH_EAST;
pub const EAST_EAST_SOUTH: i8 = EAST + SOUTH_EAST;
pub const WEST_WEST_NORTH: i8 = WEST + NORTH_WEST;
pub const WEST_WEST_SOUTH: i8 = WEST + SOUTH_WEST;
pub const BASE_FILE: u64 = 0x0101010101010101;
pub const BASE_RANK: u64 = 0xFF;
pub const FIRST_FILE: Bitboard = Bitboard::new(BASE_FILE);
pub const SECOND_FILE: Bitboard = Bitboard::new(BASE_FILE << 1);
pub const THIRD_FILE: Bitboard = Bitboard::new(BASE_FILE << 2);
pub const FOURTH_FILE: Bitboard = Bitboard::new(BASE_FILE << 3);
pub const FIFTH_FILE: Bitboard = Bitboard::new(BASE_FILE << 4);
pub const SIXTH_FILE: Bitboard = Bitboard::new(BASE_FILE << 5);
pub const SEVENTH_FILE: Bitboard = Bitboard::new(BASE_FILE << 6);
pub const EIGHTH_FILE: Bitboard = Bitboard::new(BASE_FILE << 7);
pub const FIRST_RANK: Bitboard = Bitboard::new(BASE_RANK);
pub const SECOND_RANK: Bitboard = Bitboard::new(BASE_RANK << 8);
pub const THIRD_RANK: Bitboard = Bitboard::new(BASE_RANK << 16);
pub const FOURTH_RANK: Bitboard = Bitboard::new(BASE_RANK << 24);
pub const FIFTH_RANK: Bitboard = Bitboard::new(BASE_RANK << 32);
pub const SIXTH_RANK: Bitboard = Bitboard::new(BASE_RANK << 40);
pub const SEVENTH_RANK: Bitboard = Bitboard::new(BASE_RANK << 48);
pub const EIGHTH_RANK: Bitboard = Bitboard::new(BASE_RANK << 56);

pub enum Ordinal {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

pub const FILES: [Bitboard; variant_count::<Ordinal>()] = [
    FIRST_FILE,
    SECOND_FILE,
    THIRD_FILE,
    FOURTH_FILE,
    FIFTH_FILE,
    SIXTH_FILE,
    SEVENTH_FILE,
    EIGHTH_FILE,
];

pub const RANKS: [Bitboard; variant_count::<Ordinal>()] = [
    FIRST_RANK,
    SECOND_RANK,
    THIRD_RANK,
    FOURTH_RANK,
    FIFTH_RANK,
    SIXTH_RANK,
    SEVENTH_RANK,
    EIGHTH_RANK,
];

#[derive(Default, Debug)]
pub struct Board {
    fen: ForsythEdwardsNotation,
    pieces: Pieces,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
    occupied_squares: Bitboard,
    empty_squares: Bitboard,
}

impl From<ForsythEdwardsNotation> for Board {
    fn from(value: ForsythEdwardsNotation) -> Self {
        let mut reverse_rank = 0u8;
        let mut pieces = Pieces::default();
        for rank in (0..8u8).rev() {
            let mut file = 0u8;
            for piece in value.placements().position()[reverse_rank as usize] {
                let symbol = piece as char;
                if char::is_digit(symbol, 10) {
                    file += char::to_digit(symbol, 10).unwrap() as u8;
                } else {
                    let piece = Self::init_piece(symbol, rank, file);
                    pieces.merge_piece(piece);
                }
            }
            reverse_rank += 1;
        }
        Self::default()
    }
}

impl Board {
    pub fn new(
        fen: ForsythEdwardsNotation,
        pieces: Pieces,
        white_pieces: Bitboard,
        black_pieces: Bitboard,
        occupied_squares: Bitboard,
        empty_squares: Bitboard,
    ) -> Self {
        Self {
            fen,
            pieces,
            white_pieces,
            black_pieces,
            occupied_squares,
            empty_squares,
        }
    }

    pub fn fen(&self) -> &ForsythEdwardsNotation {
        &self.fen
    }

    pub fn pieces(&self) -> &Pieces {
        &self.pieces
    }

    pub fn white_pieces(&self) -> Bitboard {
        self.white_pieces
    }

    pub fn black_pieces(&self) -> Bitboard {
        self.black_pieces
    }

    pub fn occupied_squares(&self) -> Bitboard {
        self.occupied_squares
    }

    pub fn empty_squares(&self) -> Bitboard {
        self.empty_squares
    }
}

pub trait BoardExt {
    fn init_piece(symbol: char, rank: u8, file: u8) -> Piece {
        let mut piece: Piece = Piece::try_from(symbol).unwrap();
        piece.set_on_square(rank, file);
        piece
    }
}

impl BoardExt for Board {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_file() {
        let expected = "\
            10000000\n\
            10000000\n\
            10000000\n\
            10000000\n\
            10000000\n\
            10000000\n\
            10000000\n\
            10000000\n\
            ";
        assert_eq!(expected, FIRST_FILE.to_string());
    }

    #[test]
    fn test_second_file() {
        let expected = "\
            01000000\n\
            01000000\n\
            01000000\n\
            01000000\n\
            01000000\n\
            01000000\n\
            01000000\n\
            01000000\n\
            ";
        assert_eq!(expected, SECOND_FILE.to_string());
    }

    #[test]
    fn test_third_file() {
        let expected = "\
            00100000\n\
            00100000\n\
            00100000\n\
            00100000\n\
            00100000\n\
            00100000\n\
            00100000\n\
            00100000\n\
            ";
        assert_eq!(expected, THIRD_FILE.to_string());
    }

    #[test]
    fn test_fourth_file() {
        let expected = "\
            00010000\n\
            00010000\n\
            00010000\n\
            00010000\n\
            00010000\n\
            00010000\n\
            00010000\n\
            00010000\n\
            ";
        assert_eq!(expected, FOURTH_FILE.to_string());
    }

    #[test]
    fn test_fifth_file() {
        let expected = "\
            00001000\n\
            00001000\n\
            00001000\n\
            00001000\n\
            00001000\n\
            00001000\n\
            00001000\n\
            00001000\n\
            ";
        assert_eq!(expected, FIFTH_FILE.to_string());
    }

    #[test]
    fn test_sixth_file() {
        let expected = "\
            00000100\n\
            00000100\n\
            00000100\n\
            00000100\n\
            00000100\n\
            00000100\n\
            00000100\n\
            00000100\n\
            ";
        assert_eq!(expected, SIXTH_FILE.to_string());
    }

    #[test]
    fn test_seventh_file() {
        let expected = "\
            00000010\n\
            00000010\n\
            00000010\n\
            00000010\n\
            00000010\n\
            00000010\n\
            00000010\n\
            00000010\n\
            ";
        assert_eq!(expected, SEVENTH_FILE.to_string());
    }

    #[test]
    fn test_eighth_file() {
        let expected = "\
            00000001\n\
            00000001\n\
            00000001\n\
            00000001\n\
            00000001\n\
            00000001\n\
            00000001\n\
            00000001\n\
            ";
        assert_eq!(expected, EIGHTH_FILE.to_string());
    }

    #[test]
    fn test_ranks() {
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11111111\n\
            ";
        assert_eq!(expected, FIRST_RANK.to_string());
    }

    #[test]
    fn test_second_rank() {
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11111111\n\
            00000000\n\
            ";
        assert_eq!(expected, SECOND_RANK.to_string());
    }

    #[test]
    fn test_third_rank() {
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11111111\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, THIRD_RANK.to_string());
    }

    #[test]
    fn test_fourth_rank() {
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11111111\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, FOURTH_RANK.to_string());
    }

    #[test]
    fn test_fifth_rank() {
        let expected = "\
            00000000\n\
            00000000\n\
            00000000\n\
            11111111\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, FIFTH_RANK.to_string());
    }

    #[test]
    fn test_sixth_rank() {
        let expected = "\
            00000000\n\
            00000000\n\
            11111111\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, SIXTH_RANK.to_string());
    }

    #[test]
    fn test_seventh_rank() {
        let expected = "\
            00000000\n\
            11111111\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, SEVENTH_RANK.to_string());
    }

    #[test]
    fn test_eighth_rank() {
        let expected = "\
            11111111\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            ";
        assert_eq!(expected, EIGHTH_RANK.to_string());
    }
}
