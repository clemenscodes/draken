pub(crate) mod quiet;

use quiet::QuietMove;

use super::MoveExt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReversibleMove {
    Quiet(QuietMove),
}

pub trait ReversibleMoveExt: MoveExt {}

impl ReversibleMoveExt for ReversibleMove {}

impl MoveExt for ReversibleMove {
    fn coordinates(&self) -> crate::coordinates::Coordinates {
        match *self {
            ReversibleMove::Quiet(quiet) => quiet.coordinates(),
        }
    }
}
