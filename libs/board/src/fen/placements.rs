use crate::pieces::{PIECE_BYTES, PIECE_INDEX_LOOKUP_MAP};
use std::{
    fmt::{Debug, Display},
    str::Split,
};

type Position = [[u8; NUM_FILES]; NUM_RANKS];

pub const NUM_RANKS: usize = 8;
pub const NUM_FILES: usize = 8;

const EMPTY: u8 = 0;
const B_ROOK: u8 = PIECE_BYTES[0];
const B_KNIGHT: u8 = PIECE_BYTES[1];
const B_BISHOP: u8 = PIECE_BYTES[2];
const B_QUEEN: u8 = PIECE_BYTES[3];
const B_KING: u8 = PIECE_BYTES[4];
const B_PAWN: u8 = PIECE_BYTES[5];
const W_ROOK: u8 = PIECE_BYTES[6];
const W_KNIGHT: u8 = PIECE_BYTES[7];
const W_BISHOP: u8 = PIECE_BYTES[8];
const W_QUEEN: u8 = PIECE_BYTES[9];
const W_KING: u8 = PIECE_BYTES[10];
const W_PAWN: u8 = PIECE_BYTES[11];

const STARTING_POSITION: Position = [
    [B_ROOK, B_KNIGHT, B_BISHOP, B_QUEEN, B_KING, B_BISHOP, B_KNIGHT, B_ROOK],
    [B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN],
    [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
    [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
    [W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN],
    [W_ROOK, W_KNIGHT, W_BISHOP, W_QUEEN, W_KING, W_BISHOP, W_KNIGHT, W_ROOK],
];

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Placements {
    position: Position,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlacementError {
    InvalidFile,
    InvalidPiece,
    InvalidRank,
}

impl Placements {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    fn validate_rank_count(split: &Split<'_, char>) -> Result<(), PlacementError> {
        if split.clone().count() != NUM_RANKS {
            return Err(PlacementError::InvalidRank);
        }
        Ok(())
    }

    fn process_piece(position: &mut Position, num_rank: usize, file: &mut usize, piece: char) -> Result<(), PlacementError> {
        match PIECE_INDEX_LOOKUP_MAP.get(&piece) {
            Some(&idx) => {
                position[num_rank][*file] = PIECE_BYTES[idx];
                *file += 1;
            }
            None if piece.is_ascii_digit() => {
                let empty_files = piece.to_digit(10).unwrap() as usize;
                Self::validate_file_count(*file + empty_files)?;
                for i in 0..empty_files {
                    position[num_rank][*file + i] = EMPTY;
                }
                *file += empty_files;
            }
            _ => return Err(PlacementError::InvalidPiece),
        }
        Ok(())
    }

    fn validate_file_count(file: usize) -> Result<(), PlacementError> {
        if file > NUM_FILES {
            return Err(PlacementError::InvalidFile);
        }
        Ok(())
    }
}

impl Default for Placements {
    fn default() -> Self {
        Self::new(STARTING_POSITION)
    }
}

impl TryFrom<&str> for Placements {
    type Error = PlacementError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut position: Position = [[0u8; NUM_FILES]; NUM_RANKS];
        let split = value.split('/');
        Self::validate_rank_count(&split)?;
        for (num_rank, rank) in split.enumerate() {
            let mut file = 0;
            for piece in rank.chars() {
                if file >= NUM_FILES {
                    break;
                }
                Self::process_piece(&mut position, num_rank, &mut file, piece)?;
            }
            Self::validate_file_count(file)?;
        }
        Ok(Self { position })
    }
}

impl TryFrom<String> for Placements {
    type Error = PlacementError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rank, pieces) in self.position.iter().enumerate() {
            let mut empty_files: u8 = 0;
            for &piece in pieces {
                if piece == 0 {
                    empty_files += 1;
                    continue;
                }
                if empty_files != 0 {
                    write!(f, "{empty_files}")?;
                    empty_files = 0;
                }
                write!(f, "{}", piece as char)?;
            }
            if empty_files != 0 {
                write!(f, "{empty_files}")?;
            }
            if rank < NUM_RANKS - 1 {
                write!(f, "/")?;
            }
        }
        Ok(())
    }
}

impl Debug for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_placements() {
        let placements_data = [[0; NUM_FILES]; NUM_RANKS];
        let placements = Placements::new(placements_data);
        assert_eq!(placements.position(), placements_data);
    }

    #[test]
    fn test_from_valid_string() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let chessboard = Placements::try_from(fen_string).unwrap();
        assert_eq!(chessboard.position(), STARTING_POSITION);
    }

    #[test]
    fn test_display_empty_board() {
        let empty_board = Placements::new([[0; NUM_FILES]; NUM_RANKS]);
        assert_eq!(empty_board.to_string(), "8/8/8/8/8/8/8/8");
    }

    #[test]
    fn test_display_starting_position() {
        let full_board = Placements::default();
        assert_eq!(full_board.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn test_invalid_ranks() {
        let invalid_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/extra";
        let invalid_placement_str = String::from(invalid_fen);
        let err = Placements::try_from(invalid_placement_str).unwrap_err();
        assert_eq!(err, PlacementError::InvalidRank);
    }

    #[test]
    fn test_too_many_files() {
        let invalid_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK5R";
        let invalid_placement_str = String::from(invalid_fen);
        let err = Placements::try_from(invalid_placement_str).unwrap_err();
        assert_eq!(err, PlacementError::InvalidFile);
    }

    #[test]
    fn test_invalid_piece() {
        std::panic::set_hook(Box::new(|_| {}));
        let invalid_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPJQ/RNBQKBNR";
        let err = Placements::try_from(invalid_fen).unwrap_err();
        assert_eq!(err, PlacementError::InvalidPiece);
    }
}
