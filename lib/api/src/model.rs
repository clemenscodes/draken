use crate::{ForsythEdwardsNotation, Game, Move, Square};

pub trait Model: Game + ForsythEdwardsNotation {
    fn get_fen() -> String;
    fn get_legal_moves(square: Square) -> Vec<Move>;
    fn clear_error();
}
