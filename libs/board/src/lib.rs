#![feature(variant_count)]
#![feature(lazy_cell)]
pub mod fen;
pub mod moves;
pub mod pieces;

use api::{Square, SquareExt};
use bitboard::{Bitboard, BitboardExt};
use fen::ForsythEdwardsNotation;
use pieces::{piece::Piece, Pieces, UTF_SYMBOLS};
use std::{
    fmt::{Debug, Display},
    mem::variant_count,
};

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board {
    fen: ForsythEdwardsNotation,
    pieces: Pieces,
}

pub trait BoardExt {}

pub trait Verify {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()>;
}

impl Default for Board {
    fn default() -> Self {
        let fen = ForsythEdwardsNotation::default();
        let pieces = Pieces::from(fen.placements().position());
        Self { fen, pieces }
    }
}

impl Board {
    pub fn new(fen: ForsythEdwardsNotation, pieces: Pieces) -> Self {
        Self { fen, pieces }
    }

    pub fn fen(&self) -> &ForsythEdwardsNotation {
        &self.fen
    }

    pub fn fen_mut(&mut self) -> &mut ForsythEdwardsNotation {
        &mut self.fen
    }

    pub fn pieces(&self) -> &Pieces {
        &self.pieces
    }

    pub fn pieces_mut(&mut self) -> &mut Pieces {
        &mut self.pieces
    }

    pub fn get_piece_index(&self, source: Square) -> Result<usize, ()> {
        let bitboard = Bitboard::from(source);
        let pieces = self.pieces().get_all_pieces();
        for index in 0..pieces.len() {
            if Bitboard::overlap(bitboard, pieces[index]) {
                return Ok(index);
            }
        }
        eprintln!("No piece found on {source}");
        Err(())
    }

    pub fn get_piece_mut(&mut self, source: Square) -> Result<Piece, ()> {
        let piece_index = self.get_piece_index(source).unwrap();
        let pieces = self.pieces_mut();
        match piece_index {
            0 => Ok(Piece::from(pieces.black_pieces().rook())),
            1 => Ok(Piece::from(pieces.black_pieces().knight())),
            2 => Ok(Piece::from(pieces.black_pieces().bishop())),
            3 => Ok(Piece::from(pieces.black_pieces().queen())),
            4 => Ok(Piece::from(pieces.black_pieces().king())),
            5 => Ok(Piece::from(pieces.black_pieces().pawn())),
            6 => Ok(Piece::from(pieces.white_pieces().rook())),
            7 => Ok(Piece::from(pieces.white_pieces().knight())),
            8 => Ok(Piece::from(pieces.white_pieces().bishop())),
            9 => Ok(Piece::from(pieces.white_pieces().queen())),
            10 => Ok(Piece::from(pieces.white_pieces().king())),
            11 => Ok(Piece::from(pieces.white_pieces().pawn())),
            _ => Err(()),
        }
    }

    pub fn get_piece_board_mut(&mut self, source: Square) -> Result<&mut Bitboard, ()> {
        let piece_index = self.get_piece_index(source).unwrap();
        let pieces = self.pieces_mut();
        match piece_index {
            0 => Ok(pieces.black_pieces_mut().rook_mut().bitboard_mut()),
            1 => Ok(pieces.black_pieces_mut().knight_mut().bitboard_mut()),
            2 => Ok(pieces.black_pieces_mut().bishop_mut().bitboard_mut()),
            3 => Ok(pieces.black_pieces_mut().queen_mut().bitboard_mut()),
            4 => Ok(pieces.black_pieces_mut().king_mut().bitboard_mut()),
            5 => Ok(pieces.black_pieces_mut().pawn_mut().bitboard_mut()),
            6 => Ok(pieces.white_pieces_mut().rook_mut().bitboard_mut()),
            7 => Ok(pieces.white_pieces_mut().knight_mut().bitboard_mut()),
            8 => Ok(pieces.white_pieces_mut().bishop_mut().bitboard_mut()),
            9 => Ok(pieces.white_pieces_mut().queen_mut().bitboard_mut()),
            10 => Ok(pieces.white_pieces_mut().king_mut().bitboard_mut()),
            11 => Ok(pieces.white_pieces_mut().pawn_mut().bitboard_mut()),
            _ => Err(()),
        }
    }

    pub fn capture_piece(&mut self, square: Square) -> Result<(), ()> {
        let piece = self.get_piece_board_mut(square)?;
        *piece ^= Bitboard::get_single_bit(square.into());
        Ok(())
    }
}

impl From<ForsythEdwardsNotation> for Board {
    fn from(value: ForsythEdwardsNotation) -> Self {
        let pieces = Pieces::from(value.placements().position());
        Self { fen: value, pieces }
    }
}

impl Into<Bitboard> for Board {
    fn into(self) -> Bitboard {
        let pieces = *self.pieces();
        pieces.into()
    }
}

impl BoardExt for Board {}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Square::iterate_square_indices(|rank, file| {
            let bitboard = Bitboard::try_from((rank as usize, file as usize)).unwrap();
            let symbol = self.pieces().get_piece_symbol(bitboard, UTF_SYMBOLS);
            write!(f, "[{symbol}]").unwrap();
            if file == 7 && rank != 0 {
                writeln!(f).unwrap();
            }
        });
        Ok(())
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

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

    #[test]
    fn test_create_board_from_default_fen() {
        let board = Board::from(ForsythEdwardsNotation::default());
        let expected_board = "\
        [♜][♞][♝][♛][♚][♝][♞][♜]\n\
        [♟][♟][♟][♟][♟][♟][♟][♟]\n\
        [ ][ ][ ][ ][ ][ ][ ][ ]\n\
        [ ][ ][ ][ ][ ][ ][ ][ ]\n\
        [ ][ ][ ][ ][ ][ ][ ][ ]\n\
        [ ][ ][ ][ ][ ][ ][ ][ ]\n\
        [♙][♙][♙][♙][♙][♙][♙][♙]\n\
        [♖][♘][♗][♕][♔][♗][♘][♖]\
    ";
        assert_eq!(expected_board, board.to_string());
    }

    #[test]
    fn test_empty_squares_on_board() {
        let mut board = Board::from(ForsythEdwardsNotation::default());
        let empty = board.pieces_mut().empty_squares();
        let expected_empty = "\
            00000000\n\
            00000000\n\
            11111111\n\
            11111111\n\
            11111111\n\
            11111111\n\
            00000000\n\
            00000000\n\
        ";
        assert_eq!(expected_empty, empty.to_string());
    }

    #[test]
    fn test_occupied_squares_on_board() {
        let mut board = Board::from(ForsythEdwardsNotation::default());
        let occupied = board.pieces_mut().occupied_squares();
        let expected_occupied = "\
            11111111\n\
            11111111\n\
            00000000\n\
            00000000\n\
            00000000\n\
            00000000\n\
            11111111\n\
            11111111\n\
        ";
        assert_eq!(expected_occupied, occupied.to_string());
    }
}
