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
