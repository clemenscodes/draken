use crate::Square;

pub trait MoveListExt {
    fn ply(&self) -> u16;
    fn add(&mut self, source: Square, destination: Square);
}
