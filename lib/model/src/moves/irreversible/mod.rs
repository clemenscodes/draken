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
