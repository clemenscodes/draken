use crate::moves::encoded_move::EncodedMove;
use api::{MoveListExt, Square};
use std::fmt::{Debug, Display};

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

    fn ply(&self) -> u16 {
        self.ply
    }

    fn set_ply(&mut self, ply: u16) {
        self.ply = ply;
    }

    pub fn moves(&self) -> [EncodedMove; MAX_PLY] {
        self.moves
    }

    fn moves_mut(&mut self) -> &mut Moves {
        &mut self.moves
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
            let encoded_move = self.moves()[ply as usize];
            if ply % 2 == 0 {
                let full_move = (ply / 2) + 1;
                write!(f, "{full_move}. {encoded_move}")?;
            } else {
                writeln!(f, " {encoded_move}")?;
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
        self.ply()
    }

    fn add(&mut self, data: u16) {
        let ply = self.ply();
        self.moves_mut()[ply as usize] = EncodedMove::new(data);
        self.set_ply(ply + 1);
    }

    fn validate(&self, source: Square, destination: Square) -> bool {
        source != destination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut move_list = MoveList::default();
        move_list.add(0);
        println!("{move_list}");
    }

    #[test]
    fn test_validate() {
        let mut move_list = MoveList::default();
        move_list.add(0);
    }
}
