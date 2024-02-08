use crate::{ForsythEdwardsNotationExt, GameExt, Square};

pub trait Model: GameExt + ForsythEdwardsNotationExt {
    fn get_fen(&self) -> String;
    fn get_legal_moves(&self, square: Square) -> Vec<(Square, Square)>;
    fn clear_error(&mut self);
}
