use crate::Square;

pub trait MoveListExt {
    fn ply(&self) -> u16;
    fn add(&mut self, encoded_move: u16) -> Result<(), ()>;
    fn validate(&self, source: Square, destination: Square) -> bool;
}
