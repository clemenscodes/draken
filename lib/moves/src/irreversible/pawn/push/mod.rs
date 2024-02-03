mod double;
mod single;

use double::DoublePushMove;
use single::SinglePushMove;

use crate::{irreversible::IrreversibleMoveExt, MoveExt};

use super::PawnMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushMove {
    Single(SinglePushMove),
    Double(DoublePushMove),
}

impl From<DoublePushMove> for PushMove {
    fn from(v: DoublePushMove) -> Self {
        Self::Double(v)
    }
}

impl From<SinglePushMove> for PushMove {
    fn from(v: SinglePushMove) -> Self {
        Self::Single(v)
    }
}

pub trait PushMoveExt: PawnMoveExt {}

impl PushMoveExt for PushMove {}
impl PawnMoveExt for PushMove {}
impl IrreversibleMoveExt for PushMove {}
impl MoveExt for PushMove {}
