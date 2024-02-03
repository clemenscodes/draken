use crate::{ForsythEdwardsNotationExt, GameExt, Square};

pub trait Controller: GameExt + ForsythEdwardsNotationExt {
    fn handle_mouse_pressed();
    fn handle_mouse_dragged();
    fn handle_mouse_released();
    fn handle_mouse_moved();
    fn get_error_message() -> String;
    fn clear_error_message();
    fn get_fen() -> String;
    fn get_source() -> Square;
    fn get_destination() -> Square;
    fn get_dragged_square() -> Square;
    fn get_legal_moves() -> Vec<(Square, Square)>;
}
