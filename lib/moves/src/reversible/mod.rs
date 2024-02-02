mod quiet;

use quiet::QuietMove;

use super::MoveExt;

#[derive(Debug)]
pub enum ReversibleMove {
    Quiet(QuietMove),
}

impl From<QuietMove> for ReversibleMove {
    fn from(v: QuietMove) -> Self {
        Self::Quiet(v)
    }
}

pub trait ReversibleMoveExt: MoveExt {}

impl ReversibleMoveExt for ReversibleMove {}
impl MoveExt for ReversibleMove {}
