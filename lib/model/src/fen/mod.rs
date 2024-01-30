mod active_color;
mod castling;
mod enpassant;
mod full_move_clock;
mod half_move_clock;
mod placements;

use std::fmt::{Debug, Display};

use active_color::ActiveColor;
use castling::Castling;
use enpassant::EnPassant;
use full_move_clock::FullMoveClock;
use half_move_clock::HalfMoveClock;
use placements::Placements;

pub const FEN_PARTS: usize = 6;

#[derive(Default)]
pub struct ForsythEdwardsNotation {
    placements: Placements,
    active_color: ActiveColor,
    castling: Castling,
    enpassant: EnPassant,
    half_move_clock: HalfMoveClock,
    full_move_clock: FullMoveClock,
}

impl ForsythEdwardsNotation {
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
}

#[derive(Debug)]
pub enum Error {
    InvalidParts,
    InvalidPlacements,
    InvalidActiveColor,
    InvalidCastling,
    InvalidEnPassant,
    InvalidHalfMoveClock,
    InvalidFullMoveClock,
}

impl TryFrom<&str> for ForsythEdwardsNotation {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(' ');
        if split.clone().count() != FEN_PARTS {
            return Err(Error::InvalidParts);
        }
        let parts: Vec<&str> = split.collect();
        let placements = Placements::try_from(parts[0]).map_err(|_| Error::InvalidPlacements)?;
        let active_color = ActiveColor::try_from(parts[1]).map_err(|_| Error::InvalidActiveColor)?;
        let castling = Castling::try_from(parts[2]).map_err(|_| Error::InvalidCastling)?;
        let enpassant = EnPassant::try_from(parts[3]).map_err(|_| Error::InvalidEnPassant)?;
        let half_move_clock = HalfMoveClock::try_from(parts[4]).map_err(|_| Error::InvalidHalfMoveClock)?;
        let full_move_clock = FullMoveClock::try_from(parts[5]).map_err(|_| Error::InvalidFullMoveClock)?;
        let fen = Self::new(placements, active_color, castling, enpassant, half_move_clock, full_move_clock);
        Ok(fen)
    }
}

impl TryFrom<String> for ForsythEdwardsNotation {
    type Error = Error;

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_fen() {}
}
