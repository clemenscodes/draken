pub mod coordinates;
pub mod encoded_move;
pub mod irreversible;
pub mod list;
pub mod reversible;

use board::Board;
use coordinates::Coordinates;
use irreversible::IrreversibleMove;
use reversible::ReversibleMove;

pub const QUIET_MOVE: u16 = 0b0000;
pub const DOUBLE_PAWN_PUSH: u16 = 0b0001;
pub const KING_CASTLE: u16 = 0b0010;
pub const QUEEN_CASTLE: u16 = 0b0011;
pub const CAPTURE: u16 = 0b0100;
pub const ENPASSANT: u16 = 0b0101;
pub const KNIGHT_PROMOTION: u16 = 0b1000;
pub const BISHOP_PROMOTION: u16 = 0b1001;
pub const ROOK_PROMOTION: u16 = 0b1010;
pub const QUEEN_PROMOTION: u16 = 0b1011;
pub const KNIGHT_PROMOTION_CAPTURE: u16 = 0b1100;
pub const BISHOP_PROMOTION_CAPTURE: u16 = 0b1101;
pub const ROOK_PROMOTION_CAPTURE: u16 = 0b1110;
pub const QUEEN_PROMOTION_CAPTURE: u16 = 0b1111;

const SOURCE_SHIFT: usize = 10;
const DESTINATION_SHIFT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Reversible(ReversibleMove),
    Irreversible(IrreversibleMove),
}

pub trait MoveExt {
    fn coordinates(&self) -> Coordinates;
    fn march(&self, board: &mut Board);
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

    fn march(&self, board: &mut Board) {
        match *self {
            Move::Reversible(reversible) => reversible.march(board),
            Move::Irreversible(irreversible) => irreversible.march(board),
        }
    }
}
