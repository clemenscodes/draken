use std::{
    fmt::{Debug, Display},
    str::Split,
};

use crate::pieces::{PIECE_INDEX_LOOKUP_MAP, PIECE_SYMBOLS};

pub const NUM_RANKS: usize = 8;
pub const NUM_FILES: usize = 8;

type Rank = [u8; NUM_FILES];
type Pieces = [Rank; NUM_RANKS];

pub struct Placements {
    placements: Pieces,
}

impl Placements {
    pub fn new(placements: Pieces) -> Self {
        Self { placements }
    }

    pub fn placements(&self) -> Pieces {
        self.placements
    }
}

impl Placements {
    fn validate_rank_count(split: &Split<'_, char>) {
        if split.clone().count() != NUM_RANKS {
            panic!("Invalid piece placement data, incorrect number of ranks");
        }
    }

    fn process_piece(placements: &mut Pieces, num_rank: usize, file: &mut usize, piece: char) {
        match PIECE_INDEX_LOOKUP_MAP.get(&piece) {
            Some(&idx) => {
                placements[num_rank][*file] = PIECE_SYMBOLS[idx] as u8;
                *file += 1;
            }
            None if piece.is_ascii_digit() => {
                let empty_files = piece.to_digit(10).unwrap() as usize;
                placements[num_rank][*file] = empty_files as u8;
                *file += empty_files;
            }
            _ => panic!("Invalid piece placed"),
        }
    }

    fn validate_file_count(file: usize) {
        if file > NUM_FILES {
            panic!("Invalid piece placement data, too many files");
        }
    }
}

impl From<&str> for Placements {
    fn from(value: &str) -> Self {
        let mut placements = [[0u8; NUM_FILES]; NUM_RANKS];
        let split = value.split('/');
        Self::validate_rank_count(&split);
        '_ranks: for (num_rank, rank) in split.enumerate() {
            let mut file = 0;
            '_files: for piece in rank.chars() {
                if file >= NUM_FILES {
                    break '_files;
                }
                Self::process_piece(&mut placements, num_rank, &mut file, piece);
            }
            Self::validate_file_count(file);
        }
        Self { placements }
    }
}

impl From<String> for Placements {
    fn from(value: String) -> Self {
        Placements::from(value.as_str())
    }
}

impl Display for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Debug for Placements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::PIECE_BYTES;

    use super::*;

    const EMPTY: u8 = 0u8;
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

    #[test]
    fn test_new_placements() {
        let placements_data = [[0; NUM_FILES]; NUM_RANKS];
        let placements = Placements::new(placements_data);
        assert_eq!(placements.placements(), placements_data);
    }

    #[test]
    fn test_from_valid_string() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let expected_placements = [
            [B_ROOK, B_KNIGHT, B_BISHOP, B_QUEEN, B_KING, B_BISHOP, B_KNIGHT, B_ROOK],
            [B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN, B_PAWN],
            [8, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [8, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [8, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [8, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            [W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN, W_PAWN],
            [W_ROOK, W_KNIGHT, W_BISHOP, W_QUEEN, W_KING, W_BISHOP, W_KNIGHT, W_ROOK],
        ];
        let chessboard = Placements::from(fen_string);
        assert_eq!(chessboard.placements(), expected_placements);
    }

    #[test]
    #[should_panic(expected = "Invalid piece placement data, incorrect number of ranks")]
    fn test_invalid_ranks() {
        std::panic::set_hook(Box::new(|_| {}));
        let invalid_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/extra";
        let invalid_placement_str = String::from(invalid_fen);
        let _ = Placements::from(invalid_placement_str);
    }

    #[test]
    #[should_panic(expected = "Invalid piece placement data, too many files")]
    fn test_too_many_files() {
        std::panic::set_hook(Box::new(|_| {}));
        let invalid_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK5R";
        let invalid_placement_str = String::from(invalid_fen);
        let _ = Placements::from(invalid_placement_str);
    }

    #[test]
    #[should_panic(expected = "Invalid piece placed")]
    fn test_invalid_piece() {
        std::panic::set_hook(Box::new(|_| {}));
        let invalid_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPJQ/RNBQKBNR";
        let invalid_placement_str = String::from(invalid_fen);
        let _ = Placements::from(invalid_placement_str);
    }
}
