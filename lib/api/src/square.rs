use std::convert::Into;
use Square::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

const NUM_SQUARES: usize = 64;

const SQUARES: [Square; NUM_SQUARES] = [
    A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5, G5, H5, A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
];

impl From<u32> for Square {
    fn from(index: u32) -> Self {
        assert!(index < 64, "Invalid index for square array");
        SQUARES[index as usize]
    }
}

impl Into<usize> for Square {
    fn into(self) -> usize {
        *&self as usize
    }
}

mod tests {
    #[test]
    fn test_square_into_index() {
        let index: usize = crate::Square::A1.into();
        assert_eq!(index, 0);
        let index: usize = crate::Square::H8.into();
        assert_eq!(index, 63);
    }

    #[test]
    fn test_square_from_index() {
        assert_eq!(crate::Square::A1, crate::Square::from(0));
        assert_eq!(crate::Square::H8, crate::Square::from(63));
    }

    #[test]
    #[should_panic(expected = "Invalid index for square array")]
    fn test_square_from_invalid_index() {
        std::panic::set_hook(Box::new(|_| {}));
        let _ = crate::Square::from(100);
    }
}