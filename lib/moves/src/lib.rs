pub(crate) mod coordinates;
pub(crate) mod encoded_move;
pub(crate) mod irreversible;
pub mod list;
pub(crate) mod reversible;

const SOURCE_SHIFT: usize = 10;
const DESTINATION_SHIFT: usize = 4;

use coordinates::Coordinates;
use irreversible::IrreversibleMove;
use reversible::ReversibleMove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Reversible(ReversibleMove),
    Irreversible(IrreversibleMove),
}

pub trait MoveExt {
    fn coordinates(&self) -> Coordinates;
}

pub trait Encode: MoveExt {
    fn encode(&self, kind_mask: u16) -> u16 {
        let source_index: u16 = self.coordinates().source().into();
        let destination_index: u16 = self.coordinates().destination().into();
        let source = source_index << SOURCE_SHIFT;
        let destination = destination_index << DESTINATION_SHIFT;
        let data = source | destination | kind_mask;
        data
    }
}

impl MoveExt for Move {
    fn coordinates(&self) -> Coordinates {
        match *self {
            Move::Reversible(reversible) => reversible.coordinates(),
            Move::Irreversible(irreversible) => irreversible.coordinates(),
        }
    }
}
