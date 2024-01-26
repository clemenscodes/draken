use crate::{ForsythEdwardsNotation, Game, Square};

pub trait Model: Game + ForsythEdwardsNotation {
    fn get_fen() -> String;
    fn get_legal_moves(square: Square) -> Vec<(Square, Square)>;
    fn clear_error();
}
