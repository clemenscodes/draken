mod coordinates;
mod irreversible;
mod reversible;

use irreversible::IrreversibleMove;
use reversible::ReversibleMove;

#[derive(Debug)]
pub enum Move {
    Reversible(ReversibleMove),
    Irreversible(IrreversibleMove),
}
