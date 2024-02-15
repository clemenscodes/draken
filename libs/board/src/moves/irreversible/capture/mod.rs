use super::IrreversibleMoveExt;
use crate::{
    moves::{coordinates::Coordinates, MoveExt},
    Board,
};
use api::Square;
use bitboard::{Bitboard, BitboardExt};
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CaptureMove {
    coordinates: Coordinates,
}

impl CaptureMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait CaptureMoveExt: IrreversibleMoveExt {}

impl MoveExt for CaptureMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.make(board)?;
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        let piece = board.get_piece_board_mut(source)?;
        *piece ^= Bitboard::move_mask(source, destination);
        board.capture_piece(destination)
    }
}

impl Display for CaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for CaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl CaptureMoveExt for CaptureMove {}

#[cfg(test)]
mod tests {
    use super::*;
    use api::Square::*;

    #[test]
    fn test_capture() {
        let mut board = Board::default();
        let capture_move = CaptureMove::new(E2, E7);
        capture_move.march(&mut board).unwrap();
        let expected = "\
            [♜][♞][♝][♛][♚][♝][♞][♜]\n\
            [♟][♟][♟][♟][♙][♟][♟][♟]\n\
            [ ][ ][ ][ ][ ][ ][ ][ ]\n\
            [ ][ ][ ][ ][ ][ ][ ][ ]\n\
            [ ][ ][ ][ ][ ][ ][ ][ ]\n\
            [ ][ ][ ][ ][ ][ ][ ][ ]\n\
            [♙][♙][♙][♙][ ][♙][♙][♙]\n\
            [♖][♘][♗][♕][♔][♗][♘][♖]\
        ";
        assert_eq!(expected, board.to_string());
    }
}
