use std::convert::Into;
use Square::*;

pub const NUM_RANKS: usize = 8;
pub const NUM_FILES: usize = 8;
pub const NUM_SQUARES: usize = NUM_RANKS * NUM_FILES;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

const SQUARES: [Square; NUM_SQUARES] = [
    A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5, G5, H5, A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
];

pub trait SquareExt {
    fn from_rank_file(rank: usize, file: usize) -> Square {
        assert!(rank < NUM_RANKS, "Invalid rank number");
        assert!(file < NUM_FILES, "Invalid file number");
        Square::from(Square::from_rank_file_to_index(rank, file))
    }
    fn from_rank_file_to_index(rank: usize, file: usize) -> usize {
        NUM_RANKS * rank + file
    }
    fn iterate_square_indices<F>(mut callback: F)
    where
        F: FnMut(usize, usize),
    {
        for rank in (0..8).rev() {
            for file in 0..8 {
                callback(rank, file);
            }
        }
    }
}

impl SquareExt for Square {}

impl From<usize> for Square {
    fn from(index: usize) -> Self {
        assert!(index < 64, "Invalid index for square array");
        SQUARES[index]
    }
}

impl From<u32> for Square {
    fn from(index: u32) -> Self {
        assert!(index < 64, "Invalid index for square array");
        SQUARES[index as usize]
    }
}

impl From<u16> for Square {
    fn from(index: u16) -> Self {
        assert!(index < 64, "Invalid index for square array");
        SQUARES[index as usize]
    }
}

impl From<u8> for Square {
    fn from(index: u8) -> Self {
        assert!(index < 64, "Invalid index for square array");
        SQUARES[index as usize]
    }
}

impl Into<usize> for Square {
    fn into(self) -> usize {
        *&self as usize
    }
}

impl Into<u32> for Square {
    fn into(self) -> u32 {
        *&self as u32
    }
}

impl Into<u16> for Square {
    fn into(self) -> u16 {
        *&self as u16
    }
}

impl Into<u8> for Square {
    fn into(self) -> u8 {
        *&self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_into_index_usize() {
        let index: usize = A1.into();
        assert_eq!(index, 0);
        let index: usize = H8.into();
        assert_eq!(index, 63);
    }

    #[test]
    fn test_square_into_index_u32() {
        let index: u32 = A1.into();
        assert_eq!(index, 0);
        let index: u32 = H8.into();
        assert_eq!(index, 63);
    }

    #[test]
    fn test_square_into_index_u16() {
        let index: u16 = A1.into();
        assert_eq!(index, 0);
        let index: u16 = H8.into();
        assert_eq!(index, 63);
    }

    #[test]
    fn test_square_into_index_u8() {
        let index: u8 = A1.into();
        assert_eq!(index, 0);
        let index: u8 = H8.into();
        assert_eq!(index, 63);
    }

    #[test]
    fn test_square_from_usize() {
        assert_eq!(A1, Square::from(0 as usize));
        assert_eq!(H8, Square::from(63 as usize));
    }

    #[test]
    fn test_square_from_u32() {
        assert_eq!(A1, Square::from(0 as u32));
        assert_eq!(H8, Square::from(63 as u32));
    }

    #[test]
    fn test_square_from_u16() {
        assert_eq!(A1, Square::from(0 as u16));
        assert_eq!(H8, Square::from(63 as u16));
    }

    #[test]
    fn test_square_from_u8() {
        assert_eq!(A1, Square::from(0 as u8));
        assert_eq!(H8, Square::from(63 as u8));
    }

    #[test]
    #[should_panic(expected = "Invalid index for square array")]
    fn test_square_from_invalid_index() {
        std::panic::set_hook(Box::new(|_| {}));
        let _ = Square::from(100 as usize);
    }

    #[test]
    fn test_square_from_rank_file() {
        let square = Square::from_rank_file(0, 0);
        assert_eq!(square, A1);
        let square = Square::from_rank_file(7, 7);
        assert_eq!(square, H8);
    }
}
