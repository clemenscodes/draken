use crate::{ForsythEdwardsNotationExt, GameExt, Square};

pub trait Controller: GameExt + ForsythEdwardsNotationExt {
    fn handle_mouse_pressed(&self);
    fn handle_mouse_dragged(&self);
    fn handle_mouse_released(&self);
    fn handle_mouse_moved(&self);
    fn get_error_message(&self) -> String;
    fn clear_error_message(&mut self);
    fn get_fen(&self) -> String;
    fn get_source(&self) -> Square;
    fn get_destination(&self) -> Square;
    fn get_dragged_square(&self) -> Square;
    fn get_legal_moves(&self) -> Vec<(Square, Square, Option<char>)>;
}
