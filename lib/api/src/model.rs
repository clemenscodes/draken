use crate::{ForsythEdwardsNotationExt, Game, Square};

pub trait Model: Game + ForsythEdwardsNotationExt {
    fn get_fen() -> String;
    fn get_legal_moves(square: Square) -> Vec<(Square, Square)>;
    fn clear_error();
}
