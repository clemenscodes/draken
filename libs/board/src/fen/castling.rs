use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Castling {
    rights: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CastlingError {
    Invalid,
}

impl Default for Castling {
    fn default() -> Self {
        Self::new(Self::WHITE_KING_CASTLE | Self::WHITE_QUEEN_CASTLE | Self::BLACK_KING_CASTLE | Self::BLACK_QUEEN_CASTLE)
    }
}

impl Castling {
    pub const BLACK_KING_CASTLE: u8 = 0b0010;
    pub const BLACK_QUEEN_CASTLE: u8 = 0b0001;
    pub const WHITE_KING_CASTLE: u8 = 0b1000;
    pub const WHITE_QUEEN_CASTLE: u8 = 0b0100;

    pub fn new(rights: u8) -> Self {
        Self { rights }
    }

    pub fn rights(&self) -> u8 {
        self.rights
    }

    pub fn has_white_king_castle(&self) -> bool {
        self.rights & Self::WHITE_KING_CASTLE != 0
    }

    pub fn has_white_queen_castle(&self) -> bool {
        self.rights & Self::WHITE_QUEEN_CASTLE != 0
    }

    pub fn has_black_king_castle(&self) -> bool {
        self.rights & Self::BLACK_KING_CASTLE != 0
    }

    pub fn has_black_queen_castle(&self) -> bool {
        self.rights & Self::BLACK_QUEEN_CASTLE != 0
    }

    pub fn has_no_castle_rights(&self) -> bool {
        self.rights == 0
    }
}

impl TryFrom<&str> for Castling {
    type Error = CastlingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut rights = 0;
        if value.len() > 4 {
            return Err(Self::Error::Invalid);
        }
        for char in value.chars() {
            match char {
                'K' => rights |= Self::WHITE_KING_CASTLE,
                'Q' => rights |= Self::WHITE_QUEEN_CASTLE,
                'k' => rights |= Self::BLACK_KING_CASTLE,
                'q' => rights |= Self::BLACK_QUEEN_CASTLE,
                '-' => return Ok(Castling::default()),
                _ => return Err(Self::Error::Invalid),
            }
        }
        Ok(Castling::new(rights))
    }
}

impl TryFrom<String> for Castling {
    type Error = CastlingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_white_king_castle() {
            write!(f, "K")?;
        }
        if self.has_white_queen_castle() {
            write!(f, "Q")?;
        }
        if self.has_black_king_castle() {
            write!(f, "k")?;
        }
        if self.has_black_queen_castle() {
            write!(f, "q")?;
        }
        if self.has_no_castle_rights() {
            write!(f, "-")?;
        }
        Ok(())
    }
}

impl Debug for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_castling_rights() {
        let castling = Castling::default();
        assert_eq!(
            castling.rights(),
            Castling::WHITE_KING_CASTLE | Castling::WHITE_QUEEN_CASTLE | Castling::BLACK_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE
        );
    }

    #[test]
    fn test_custom_castling_rights() {
        let custom_rights = Castling::WHITE_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE;
        let castling = Castling::new(custom_rights);
        assert_eq!(castling.rights(), custom_rights);
    }

    #[test]
    fn test_has_white_king_castle() {
        let castling = Castling::default();
        assert!(castling.has_white_king_castle());
        let castling_without_white_king =
            Castling::new(Castling::WHITE_QUEEN_CASTLE | Castling::BLACK_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE);
        assert!(!castling_without_white_king.has_white_king_castle());
    }

    #[test]
    fn test_has_white_queen_castle() {
        let castling = Castling::default();
        assert!(castling.has_white_queen_castle());
        let castling_without_white_queen =
            Castling::new(Castling::WHITE_KING_CASTLE | Castling::BLACK_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE);
        assert!(!castling_without_white_queen.has_white_queen_castle());
    }

    #[test]
    fn test_has_black_king_castle() {
        let castling = Castling::default();
        assert!(castling.has_black_king_castle());
        let castling_without_black_king =
            Castling::new(Castling::WHITE_KING_CASTLE | Castling::WHITE_QUEEN_CASTLE | Castling::BLACK_QUEEN_CASTLE);
        assert!(!castling_without_black_king.has_black_king_castle());
    }

    #[test]
    fn test_has_black_queen_castle() {
        let castling = Castling::default();
        assert!(castling.has_black_queen_castle());
        let castling_without_black_queen =
            Castling::new(Castling::WHITE_KING_CASTLE | Castling::WHITE_QUEEN_CASTLE | Castling::BLACK_KING_CASTLE);
        assert!(!castling_without_black_queen.has_black_queen_castle());
    }

    #[test]
    fn test_display_with_castling_rights() {
        let castling = Castling::new(Castling::WHITE_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE);
        assert_eq!(format!("{castling}"), "Kq");
    }

    #[test]
    fn test_display_with_all_castling_rights() {
        let castling = Castling::default();
        assert_eq!(format!("{castling}"), "KQkq");
    }

    #[test]
    fn test_individual_castling_rights() {
        let castling = Castling::new(Castling::BLACK_KING_CASTLE);
        assert_eq!(format!("{castling}"), "k");
    }

    #[test]
    fn test_combination_of_castling_rights() {
        let castling = Castling::new(Castling::WHITE_KING_CASTLE | Castling::BLACK_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE);
        assert_eq!(format!("{castling}"), "Kkq");
    }

    #[test]
    fn test_display_without_castling_rights() {
        let castling = Castling::new(0);
        assert_eq!(format!("{castling}"), "-");
    }

    #[test]
    fn test_try_from_str_default() {
        let result = Castling::try_from("-");
        assert_eq!(result, Ok(Castling::default()));
    }

    #[test]
    fn test_try_from_str_valid_rights() {
        let result = Castling::try_from("KQkq");
        assert_eq!(
            result,
            Ok(Castling::new(
                Castling::WHITE_KING_CASTLE | Castling::WHITE_QUEEN_CASTLE | Castling::BLACK_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE
            ))
        );
    }

    #[test]
    fn test_try_from_str_invalid_length() {
        let result = Castling::try_from("KQkqX");
        assert_eq!(result, Err(CastlingError::Invalid));
    }

    #[test]
    fn test_try_from_str_invalid_character() {
        let result = Castling::try_from("KQkx");
        assert_eq!(result, Err(CastlingError::Invalid));
    }

    #[test]
    fn test_try_from_string() {
        let result = Castling::try_from(String::from("KQkq"));
        assert_eq!(
            result,
            Ok(Castling::new(
                Castling::WHITE_KING_CASTLE | Castling::WHITE_QUEEN_CASTLE | Castling::BLACK_KING_CASTLE | Castling::BLACK_QUEEN_CASTLE
            ))
        );
    }

    #[test]
    fn test_try_from_string_invalid() {
        let result = Castling::try_from(String::from("invalid"));
        assert_eq!(result, Err(CastlingError::Invalid));
    }
}
