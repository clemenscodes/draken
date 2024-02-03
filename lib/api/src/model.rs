use crate::{ForsythEdwardsNotationExt, GameExt, Square};

pub trait Model: GameExt + ForsythEdwardsNotationExt {
    fn get_fen() -> String;
    fn get_legal_moves(square: Square) -> Vec<(Square, Square)>;
    fn clear_error();
}
