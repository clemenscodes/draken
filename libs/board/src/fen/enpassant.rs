use api::{Square, Square::*};
use bitboard::{Bitboard, BitboardExt};
use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct EnPassant {
    square: Option<Square>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnPassantError {
    Invalid,
}

pub trait EnPassantExt {
    fn set_square(&mut self, square: Option<Square>);
    fn unset(&mut self);
}

impl EnPassant {
    pub fn new(square: Option<Square>) -> Self {
        if Self::is_valid_square(square) {
            Self { square }
        } else {
            panic!("Invalid en passant square");
        }
    }

    fn is_valid_square(square: Option<Square>) -> bool {
        if let Some(sq) = square {
            (sq >= A3 && sq <= H3) || (sq >= A6 && sq <= H6)
        } else {
            true
        }
    }

    pub fn square(&self) -> Option<Square> {
        self.square
    }

    pub fn mask(&self) -> Bitboard {
        if let Some(square) = self.square() {
            Bitboard::get_single_bit(square.into())
        } else {
            Bitboard::default()
        }
    }
}

impl TryFrom<&str> for EnPassant {
    type Error = EnPassantError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "-" {
            return Ok(EnPassant::default());
        }
        match Square::try_from(value) {
            Ok(square) if EnPassant::is_valid_square(Some(square)) => Ok(EnPassant { square: Some(square) }),
            _ => Err(Self::Error::Invalid),
        }
    }
}

impl TryFrom<String> for EnPassant {
    type Error = EnPassantError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl EnPassantExt for EnPassant {
    fn set_square(&mut self, square: Option<Square>) {
        if Self::is_valid_square(square) {
            self.square = square;
        }
    }

    fn unset(&mut self) {
        self.square = None;
    }
}

impl Display for EnPassant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let square = self.square.map_or(String::from("-"), |square| square.to_string());
        write!(f, "{}", square)
    }
}

impl Debug for EnPassant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::Square::{A2, A3, H7};

    #[test]
    fn test_new_en_passant_with_square() {
        let square = Some(A3);
        let en_passant = EnPassant::new(square);
        assert_eq!(en_passant.square(), square);
    }

    #[test]
    fn test_new_en_passant_default() {
        let en_passant = EnPassant::default();
        assert_eq!(en_passant.square(), None);
    }

    #[test]
    #[should_panic(expected = "Invalid en passant square")]
    fn test_invalid_en_passant_square() {
        std::panic::set_hook(Box::new(|_| {}));
        EnPassant::new(Some(H7));
    }

    #[test]
    #[should_panic(expected = "Invalid en passant square")]
    fn test_invalid_en_passant_square_out_of_range() {
        std::panic::set_hook(Box::new(|_| {}));
        EnPassant::new(Some(A2));
    }

    #[test]
    fn test_display_en_passant_with_square() {
        let square = Some(A3);
        let en_passant = EnPassant::new(square);
        assert_eq!(format!("{en_passant}"), square.unwrap().to_string());
    }

    #[test]
    fn test_display_en_passant_default() {
        let en_passant = EnPassant::default();
        assert_eq!(format!("{en_passant}"), String::from("-"));
    }

    #[test]
    fn test_debug_en_passant() {
        let square = Some(A3);
        let en_passant = EnPassant::new(square);
        assert_eq!(format!("{en_passant:?}"), String::from("a3"));
    }

    #[test]
    fn test_try_from_str_default() {
        let result = EnPassant::try_from("-");
        assert_eq!(result, Ok(EnPassant::default()));
    }

    #[test]
    fn test_try_from_str_invalid_square() {
        let result = EnPassant::try_from("x9");
        assert_eq!(result, Err(EnPassantError::Invalid));
    }

    #[test]
    fn test_try_from_string() {
        let result = EnPassant::try_from(String::from("h3"));
        assert_eq!(result, Ok(EnPassant { square: Some(Square::H3) }));
    }

    #[test]
    fn test_try_from_string_invalid() {
        let result = EnPassant::try_from("e4");
        assert_eq!(result, Err(EnPassantError::Invalid));
    }
}
