mod capture;
mod castle;
mod pawn;

use capture::CaptureMove;
use castle::CastleMove;
use pawn::PawnMove;

#[derive(Debug)]
pub enum IrreversibleMove {
    Capture(CaptureMove),
    Pawn(PawnMove),
    Castle(CastleMove),
}

impl From<CastleMove> for IrreversibleMove {
    fn from(v: CastleMove) -> Self {
        Self::Castle(v)
    }
}

impl From<PawnMove> for IrreversibleMove {
    fn from(v: PawnMove) -> Self {
        Self::Pawn(v)
    }
}

impl From<CaptureMove> for IrreversibleMove {
    fn from(v: CaptureMove) -> Self {
        Self::Capture(v)
    }
}
