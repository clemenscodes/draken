mod capture;
mod enpassant;
mod promotion;
mod push;

use capture::PawnCaptureMove;
use enpassant::EnPassantMove;
use promotion::PromotionMove;
use push::PushMove;

#[derive(Debug)]
pub enum PawnMove {
    Push(PushMove),
    Capture(PawnCaptureMove),
    EnPassant(EnPassantMove),
    Promotion(PromotionMove),
}
