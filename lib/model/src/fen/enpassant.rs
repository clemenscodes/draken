use std::fmt::{Debug, Display};

use api::Square;
use api::Square::{A3, A6, H3, H6};

pub struct EnPassant {
    square: Option<Square>,
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
}

pub(crate) trait EnPassantExt {
    fn set_square(&mut self, square: Option<Square>);
    fn unset(&mut self);
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

impl Default for EnPassant {
    fn default() -> Self {
        Self { square: None }
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
}
