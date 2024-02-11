use crate::{
    moves::{coordinates::Coordinates, Encode, MoveExt},
    Board,
};

use super::ReversibleMoveExt;
use api::Square;
use bitboard::{Bitboard, BitboardExt};
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QuietMove {
    coordinates: Coordinates,
}

impl QuietMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QuietMoveExt: ReversibleMoveExt {}

impl QuietMoveExt for QuietMove {}
impl ReversibleMoveExt for QuietMove {}

impl MoveExt for QuietMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) {
        self.increment_half_move_clock(board);
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        let piece = board.get_piece_board_mut(source).unwrap();
        *piece ^= Bitboard::move_mask(source, destination);
    }
}

impl Encode for QuietMove {}

impl Display for QuietMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for QuietMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::Square::*;

    #[test]
    fn test_quiet_move() {
        let mut board = Board::default();
        let quiet_move = QuietMove::new(E2, E4);
        quiet_move.march(&mut board);
        let expected = "\
            [♜][♞][♝][♛][♚][♝][♞][♜]\n\
            [♟][♟][♟][♟][♟][♟][♟][♟]\n\
            [ ][ ][ ][ ][ ][ ][ ][ ]\n\
            [ ][ ][ ][ ][ ][ ][ ][ ]\n\
            [ ][ ][ ][ ][♙][ ][ ][ ]\n\
            [ ][ ][ ][ ][ ][ ][ ][ ]\n\
            [♙][♙][♙][♙][ ][♙][♙][♙]\n\
            [♖][♘][♗][♕][♔][♗][♘][♖]\
        ";
        assert_eq!(expected, board.to_string());
    }
}
