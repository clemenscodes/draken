pub(crate) mod coordinates;
pub(crate) mod irreversible;
pub mod list;
pub(crate) mod reversible;

use irreversible::IrreversibleMove;
use reversible::ReversibleMove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Reversible(ReversibleMove),
    Irreversible(IrreversibleMove),
}

impl From<IrreversibleMove> for Move {
    fn from(v: IrreversibleMove) -> Self {
        Self::Irreversible(v)
    }
}

impl From<ReversibleMove> for Move {
    fn from(v: ReversibleMove) -> Self {
        Self::Reversible(v)
    }
}

pub trait MoveExt {}

impl MoveExt for Move {}
