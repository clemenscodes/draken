use std::fmt::{Debug, Display};
use Square::*;

pub const NUM_RANKS: usize = 8;
pub const NUM_FILES: usize = 8;

#[rustfmt::skip]
const SQUARES: [Square; std::mem::variant_count::<Square>()] = [
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
];

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SquareError {
    Invalid,
}

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
    where F: FnMut(usize, usize) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                callback(rank, file);
            }
        }
    }
}

#[rustfmt::skip]
impl TryFrom<&str> for Square {
    type Error = SquareError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(Self::Error::Invalid);
        }
        match value {
            "a1" => Ok(A1), "b1" => Ok(B1), "c1" => Ok(C1), "d1" => Ok(D1),
            "e1" => Ok(E1), "f1" => Ok(F1), "g1" => Ok(G1), "h1" => Ok(H1),
            "a2" => Ok(A2), "b2" => Ok(B2), "c2" => Ok(C2), "d2" => Ok(D2),
            "e2" => Ok(E2), "f2" => Ok(F2), "g2" => Ok(G2), "h2" => Ok(H2),
            "a3" => Ok(A3), "b3" => Ok(B3), "c3" => Ok(C3), "d3" => Ok(D3),
            "e3" => Ok(E3), "f3" => Ok(F3), "g3" => Ok(G3), "h3" => Ok(H3),
            "a4" => Ok(A4), "b4" => Ok(B4), "c4" => Ok(C4), "d4" => Ok(D4),
            "e4" => Ok(E4), "f4" => Ok(F4), "g4" => Ok(G4), "h4" => Ok(H4),
            "a5" => Ok(A5), "b5" => Ok(B5), "c5" => Ok(C5), "d5" => Ok(D5),
            "e5" => Ok(E5), "f5" => Ok(F5), "g5" => Ok(G5), "h5" => Ok(H5),
            "a6" => Ok(A6), "b6" => Ok(B6), "c6" => Ok(C6), "d6" => Ok(D6),
            "e6" => Ok(E6), "f6" => Ok(F6), "g6" => Ok(G6), "h6" => Ok(H6),
            "a7" => Ok(A7), "b7" => Ok(B7), "c7" => Ok(C7), "d7" => Ok(D7),
            "e7" => Ok(E7), "f7" => Ok(F7), "g7" => Ok(G7), "h7" => Ok(H7),
            "a8" => Ok(A8), "b8" => Ok(B8), "c8" => Ok(C8), "d8" => Ok(D8),
            "e8" => Ok(E8), "f8" => Ok(F8), "g8" => Ok(G8), "h8" => Ok(H8),
            _ => Err(Self::Error::Invalid),
        }
    }
}

impl SquareExt for Square {}

impl From<(usize, usize)> for Square {
    fn from((rank, file): (usize, usize)) -> Self {
        assert!(rank * file < 64, "Invalid index for square array");
        SQUARES[8 * rank + file]
    }
}

impl From<usize> for Square {
    fn from(index: usize) -> Self {
        assert!(index < 64, "Invalid index for square array");
        SQUARES[index]
    }
}

impl From<u64> for Square {
    fn from(index: u64) -> Self {
        assert!(index < 64, "Invalid index for square array");
        SQUARES[index as usize]
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

impl From<Square> for usize {
    fn from(index: Square) -> Self {
        index as usize
    }
}

impl From<Square> for u64 {
    fn from(index: Square) -> Self {
        index as u64
    }
}

impl From<Square> for u32 {
    fn from(index: Square) -> Self {
        index as u32
    }
}

impl From<Square> for u16 {
    fn from(index: Square) -> Self {
        index as u16
    }
}

impl From<Square> for u8 {
    fn from(index: Square) -> Self {
        index as u8
    }
}

#[rustfmt::skip]
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            A1 => write!(f, "a1"), B1 => write!(f, "b1"), C1 => write!(f, "c1"), D1 => write!(f, "d1"),
            E1 => write!(f, "e1"), F1 => write!(f, "f1"), G1 => write!(f, "g1"), H1 => write!(f, "h1"),
            A2 => write!(f, "a2"), B2 => write!(f, "b2"), C2 => write!(f, "c2"), D2 => write!(f, "d2"),
            E2 => write!(f, "e2"), F2 => write!(f, "f2"), G2 => write!(f, "g2"), H2 => write!(f, "h2"),
            A3 => write!(f, "a3"), B3 => write!(f, "b3"), C3 => write!(f, "c3"), D3 => write!(f, "d3"),
            E3 => write!(f, "e3"), F3 => write!(f, "f3"), G3 => write!(f, "g3"), H3 => write!(f, "h3"),
            A4 => write!(f, "a4"), B4 => write!(f, "b4"), C4 => write!(f, "c4"), D4 => write!(f, "d4"),
            E4 => write!(f, "e4"), F4 => write!(f, "f4"), G4 => write!(f, "g4"), H4 => write!(f, "h4"),
            A5 => write!(f, "a5"), B5 => write!(f, "b5"), C5 => write!(f, "c5"), D5 => write!(f, "d5"),
            E5 => write!(f, "e5"), F5 => write!(f, "f5"), G5 => write!(f, "g5"), H5 => write!(f, "h5"),
            A6 => write!(f, "a6"), B6 => write!(f, "b6"), C6 => write!(f, "c6"), D6 => write!(f, "d6"),
            E6 => write!(f, "e6"), F6 => write!(f, "f6"), G6 => write!(f, "g6"), H6 => write!(f, "h6"),
            A7 => write!(f, "a7"), B7 => write!(f, "b7"), C7 => write!(f, "c7"), D7 => write!(f, "d7"),
            E7 => write!(f, "e7"), F7 => write!(f, "f7"), G7 => write!(f, "g7"), H7 => write!(f, "h7"),
            A8 => write!(f, "a8"), B8 => write!(f, "b8"), C8 => write!(f, "c8"), D8 => write!(f, "d8"),
            E8 => write!(f, "e8"), F8 => write!(f, "f8"), G8 => write!(f, "g8"), H8 => write!(f, "h8"),
        }
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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
