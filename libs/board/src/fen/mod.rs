pub mod active_color;
pub mod castling;
pub mod enpassant;
pub mod full_move_clock;
pub mod half_move_clock;
pub mod placements;

use self::{active_color::*, castling::*, enpassant::*, full_move_clock::*, half_move_clock::*, placements::*};
use api::ForsythEdwardsNotationExt;
use bitboard::Bitboard;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ForsythEdwardsNotation {
    placements: Placements,
    active_color: ActiveColor,
    castling: Castling,
    enpassant: EnPassant,
    half_move_clock: HalfMoveClock,
    full_move_clock: FullMoveClock,
}

impl ForsythEdwardsNotation {
    pub const PARTS: usize = 6;

    pub fn new(
        placements: Placements,
        active_color: ActiveColor,
        castling: Castling,
        enpassant: EnPassant,
        half_move_clock: HalfMoveClock,
        full_move_clock: FullMoveClock,
    ) -> Self {
        Self {
            placements,
            active_color,
            castling,
            enpassant,
            half_move_clock,
            full_move_clock,
        }
    }

    pub fn placements(&self) -> &Placements {
        &self.placements
    }

    pub fn active_color(&self) -> &ActiveColor {
        &self.active_color
    }

    pub fn castling(&self) -> &Castling {
        &self.castling
    }

    pub fn enpassant(&self) -> &EnPassant {
        &self.enpassant
    }

    pub fn half_move_clock(&self) -> &HalfMoveClock {
        &self.half_move_clock
    }

    pub fn full_move_clock(&self) -> &FullMoveClock {
        &self.full_move_clock
    }

    pub fn placements_mut(&mut self) -> &mut Placements {
        &mut self.placements
    }

    pub fn active_color_mut(&mut self) -> &mut ActiveColor {
        &mut self.active_color
    }

    pub fn castling_mut(&mut self) -> &mut Castling {
        &mut self.castling
    }

    pub fn enpassant_mut(&mut self) -> &mut EnPassant {
        &mut self.enpassant
    }

    pub fn half_move_clock_mut(&mut self) -> &mut HalfMoveClock {
        &mut self.half_move_clock
    }

    pub fn full_move_clock_mut(&mut self) -> &mut FullMoveClock {
        &mut self.full_move_clock
    }

    pub fn enpassant_mask(&self) -> Bitboard {
        self.enpassant().mask()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ForsythEdwardsNotationError {
    Invalid,
    InvalidPlacements(PlacementError),
    InvalidActiveColor(ActiveColorError),
    InvalidCastling(CastlingError),
    InvalidEnPassant(EnPassantError),
    InvalidHalfMoveClock(HalfMoveClockError),
    InvalidFullMoveClock(FullMoveClockError),
}

impl Display for ForsythEdwardsNotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForsythEdwardsNotationError::Invalid => write!(f, "Invalid fen"),
            ForsythEdwardsNotationError::InvalidPlacements(_) => write!(f, "Invalid placement data"),
            ForsythEdwardsNotationError::InvalidActiveColor(_) => write!(f, "Invalid active color"),
            ForsythEdwardsNotationError::InvalidCastling(_) => write!(f, "Invalid castling information"),
            ForsythEdwardsNotationError::InvalidEnPassant(_) => write!(f, "Invalid en passant square"),
            ForsythEdwardsNotationError::InvalidHalfMoveClock(_) => write!(f, "Invalid half move clock"),
            ForsythEdwardsNotationError::InvalidFullMoveClock(_) => write!(f, "Invalid full move clock"),
        }
    }
}

impl Error for ForsythEdwardsNotationError {}

impl From<PlacementError> for ForsythEdwardsNotationError {
    fn from(value: PlacementError) -> Self {
        Self::InvalidPlacements(value)
    }
}

impl From<ActiveColorError> for ForsythEdwardsNotationError {
    fn from(value: ActiveColorError) -> Self {
        Self::InvalidActiveColor(value)
    }
}

impl From<CastlingError> for ForsythEdwardsNotationError {
    fn from(value: CastlingError) -> Self {
        Self::InvalidCastling(value)
    }
}

impl From<EnPassantError> for ForsythEdwardsNotationError {
    fn from(value: EnPassantError) -> Self {
        Self::InvalidEnPassant(value)
    }
}

impl From<HalfMoveClockError> for ForsythEdwardsNotationError {
    fn from(value: HalfMoveClockError) -> Self {
        Self::InvalidHalfMoveClock(value)
    }
}

impl From<FullMoveClockError> for ForsythEdwardsNotationError {
    fn from(value: FullMoveClockError) -> Self {
        Self::InvalidFullMoveClock(value)
    }
}

impl TryFrom<&str> for ForsythEdwardsNotation {
    type Error = ForsythEdwardsNotationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(' ').collect();
        if parts.len() != Self::PARTS {
            return Err(Self::Error::Invalid);
        }
        let placements = Placements::try_from(parts[0])?;
        let active_color = ActiveColor::try_from(parts[1])?;
        let castling = Castling::try_from(parts[2])?;
        let enpassant = EnPassant::try_from(parts[3])?;
        let half_move_clock = HalfMoveClock::try_from(parts[4])?;
        let full_move_clock = FullMoveClock::try_from(parts[5])?;
        let fen = Self::new(placements, active_color, castling, enpassant, half_move_clock, full_move_clock);
        Ok(fen)
    }
}

impl TryFrom<String> for ForsythEdwardsNotation {
    type Error = ForsythEdwardsNotationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for ForsythEdwardsNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let placements = self.placements();
        let color = self.active_color();
        let castling = self.castling();
        let enpassant = self.enpassant();
        let half = self.half_move_clock();
        let full = self.full_move_clock();
        writeln!(f, "{placements} {color} {castling} {enpassant} {half} {full}",)
    }
}

impl Debug for ForsythEdwardsNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl ForsythEdwardsNotationExt for ForsythEdwardsNotation {
    fn get_piece_placement_data(&self) -> Vec<String> {
        self.placements().to_string().split('/').map(String::from).collect()
    }

    fn is_white(&self) -> bool {
        self.active_color().is_white()
    }

    fn get_castling(&self) -> String {
        todo!()
    }

    fn get_white_king_castle(&self) -> bool {
        todo!()
    }

    fn get_white_queen_castle(&self) -> bool {
        todo!()
    }

    fn get_black_king_castle(&self) -> bool {
        todo!()
    }

    fn get_black_queen_castle(&self) -> bool {
        todo!()
    }

    fn get_en_passant(&self) -> String {
        todo!()
    }

    fn get_half_move_clock(&self) -> u8 {
        todo!()
    }

    fn get_full_move_clock(&self) -> u16 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_valid_fen() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let result = ForsythEdwardsNotation::try_from(fen_str);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_from_invalid_fen() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let result = ForsythEdwardsNotation::try_from(String::from(fen_str));
        assert_eq!(result, Err(ForsythEdwardsNotationError::Invalid));
    }

    #[test]
    fn test_display_format() {
        let fen = ForsythEdwardsNotation::default();
        let display_str = format!("{}", fen);
        assert_eq!(display_str, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\n");
    }

    #[test]
    fn test_debug_format() {
        let fen = ForsythEdwardsNotation::default();
        let debug_str = format!("{:?}", fen);
        assert_eq!(debug_str, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\n");
    }

    #[test]
    fn test_try_from_invalid_parts() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        let result = ForsythEdwardsNotation::try_from(fen_str);
        assert_eq!(
            result,
            Err(ForsythEdwardsNotationError::InvalidActiveColor(ActiveColorError::Invalid))
        );
    }
}
