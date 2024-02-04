use std::fmt::{Debug, Display};

use api::MoveListExt;

pub const MAX_PLY: usize = 512;

type Moves = [u16; MAX_PLY];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MoveList {
    moves: Moves,
    ply: u16,
}

impl MoveList {
    pub fn new(moves: Moves, ply: u16) -> Self {
        Self { moves, ply }
    }

    pub fn moves(&self) -> &[u16] {
        self.moves.as_ref()
    }

    pub fn set_ply(&mut self, ply: u16) {
        self.ply = ply;
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [0; MAX_PLY],
            ply: 0,
        }
    }
}

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Move list!")?;
        for ply in 0..self.ply() {
            let action = self.moves()[ply as usize];
            writeln!(f, "Ply: {ply} {action}")?;
        }
        Ok(())
    }
}

impl Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl MoveListExt for MoveList {
    fn ply(&self) -> u16 {
        self.ply
    }

    fn add(&mut self, source: api::Square, destination: api::Square) {
        println!("Making move from {source} to {destination}");
        self.set_ply(self.ply() + 1)
    }
}
