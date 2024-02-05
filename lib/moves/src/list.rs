use std::fmt::{Debug, Display};

use api::MoveListExt;

use crate::encoded_move::EncodedMove;

pub const MAX_PLY: usize = 512;

type Moves = [EncodedMove; MAX_PLY];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MoveList {
    ply: u16,
    moves: Moves,
}

impl MoveList {
    pub fn new(ply: u16, moves: Moves) -> Self {
        Self { ply, moves }
    }

    pub fn set_ply(&mut self, ply: u16) {
        self.ply = ply;
    }

    pub fn moves(&self) -> [EncodedMove; MAX_PLY] {
        self.moves
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [EncodedMove::default(); MAX_PLY],
            ply: 0,
        }
    }
}

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Move list!")?;
        for ply in 0..self.ply() {
            let action = self.moves()[ply as usize];
            if ply % 2 == 0 {
                let full_move = (ply / 2) + 1;
                write!(f, "{full_move}. {action}")?;
            } else {
                writeln!(f, " {action}")?;
            }
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

    fn add(&mut self, source: api::Square, destination: api::Square) -> Result<(), ()> {
        println!("Making move from {source} to {destination}");
        if self.validate(source, destination) {
            self.set_ply(self.ply() + 1);
            Ok(())
        } else {
            Err(())
        }
    }

    fn validate(&self, source: api::Square, destination: api::Square) -> bool {
        source != destination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut move_list = MoveList::default();
        move_list.add(api::Square::A1, api::Square::A2).unwrap();
        println!("{move_list}");
    }

    #[test]
    fn test_validate() {
        let mut move_list = MoveList::default();
        move_list.add(api::Square::A1, api::Square::A1).unwrap_err();
    }
}
