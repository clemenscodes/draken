use std::fmt::{Debug, Display};

const WHITE_KING_CASTLE: u8 = 0b1000;
const WHITE_QUEEN_CASTLE: u8 = 0b0100;
const BLACK_KING_CASTLE: u8 = 0b0010;
const BLACK_QUEEN_CASTLE: u8 = 0b0001;

pub struct Castling {
    rights: u8,
}

impl Default for Castling {
    fn default() -> Self {
        Self::new(WHITE_KING_CASTLE | WHITE_QUEEN_CASTLE | BLACK_KING_CASTLE | BLACK_QUEEN_CASTLE)
    }
}

impl Castling {
    pub fn new(rights: u8) -> Self {
        Self { rights }
    }

    pub fn rights(&self) -> u8 {
        self.rights
    }

    pub fn has_white_king_castle(&self) -> bool {
        self.rights & WHITE_KING_CASTLE != 0
    }

    pub fn has_white_queen_castle(&self) -> bool {
        self.rights & WHITE_QUEEN_CASTLE != 0
    }

    pub fn has_black_king_castle(&self) -> bool {
        self.rights & BLACK_KING_CASTLE != 0
    }

    pub fn has_black_queen_castle(&self) -> bool {
        self.rights & BLACK_QUEEN_CASTLE != 0
    }

    pub fn has_no_castle_rights(&self) -> bool {
        self.rights == 0
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
            WHITE_KING_CASTLE | WHITE_QUEEN_CASTLE | BLACK_KING_CASTLE | BLACK_QUEEN_CASTLE
        );
    }

    #[test]
    fn test_custom_castling_rights() {
        let custom_rights = WHITE_KING_CASTLE | BLACK_QUEEN_CASTLE;
        let castling = Castling::new(custom_rights);
        assert_eq!(castling.rights(), custom_rights);
    }

    #[test]
    fn test_has_white_king_castle() {
        let castling = Castling::default();
        assert!(castling.has_white_king_castle());
        let castling_without_white_king = Castling::new(WHITE_QUEEN_CASTLE | BLACK_KING_CASTLE | BLACK_QUEEN_CASTLE);
        assert!(!castling_without_white_king.has_white_king_castle());
    }

    #[test]
    fn test_has_white_queen_castle() {
        let castling = Castling::default();
        assert!(castling.has_white_queen_castle());
        let castling_without_white_queen = Castling::new(WHITE_KING_CASTLE | BLACK_KING_CASTLE | BLACK_QUEEN_CASTLE);
        assert!(!castling_without_white_queen.has_white_queen_castle());
    }

    #[test]
    fn test_has_black_king_castle() {
        let castling = Castling::default();
        assert!(castling.has_black_king_castle());
        let castling_without_black_king = Castling::new(WHITE_KING_CASTLE | WHITE_QUEEN_CASTLE | BLACK_QUEEN_CASTLE);
        assert!(!castling_without_black_king.has_black_king_castle());
    }

    #[test]
    fn test_has_black_queen_castle() {
        let castling = Castling::default();
        assert!(castling.has_black_queen_castle());
        let castling_without_black_queen = Castling::new(WHITE_KING_CASTLE | WHITE_QUEEN_CASTLE | BLACK_KING_CASTLE);
        assert!(!castling_without_black_queen.has_black_queen_castle());
    }

    #[test]
    fn test_display_with_castling_rights() {
        let castling = Castling::new(WHITE_KING_CASTLE | BLACK_QUEEN_CASTLE);
        assert_eq!(format!("{castling}"), "Kq");
    }

    #[test]
    fn test_display_with_all_castling_rights() {
        let castling = Castling::default();
        assert_eq!(format!("{castling}"), "KQkq");
    }

    #[test]
    fn test_individual_castling_rights() {
        let castling = Castling::new(BLACK_KING_CASTLE);
        assert_eq!(format!("{castling}"), "k");
    }

    #[test]
    fn test_combination_of_castling_rights() {
        let castling = Castling::new(WHITE_KING_CASTLE | BLACK_KING_CASTLE | BLACK_QUEEN_CASTLE);
        assert_eq!(format!("{castling}"), "Kkq");
    }

    #[test]
    fn test_display_without_castling_rights() {
        let castling = Castling::new(0);
        assert_eq!(format!("{castling}"), "-");
    }
}
