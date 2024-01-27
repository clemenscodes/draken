use api::Square;

#[derive(Debug)]
pub struct Coordinates {
    source: Square,
    destination: Square,
}

#[derive(Debug)]
pub enum Move {
    Reversible(ReversibleMove),
    Irreversible(IrreversibleMove),
}

#[derive(Debug)]
pub enum ReversibleMove {
    Quiet(QuietMove),
}

#[derive(Debug)]
pub enum IrreversibleMove {
    Capture(CaptureMove),
    Pawn(PawnMove),
    Castle(CastleMove),
}

#[derive(Debug)]
pub struct QuietMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct CaptureMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub enum PawnMove {
    Push(PushMove),
    Capture(PawnCaptureMove),
    EnPassant(EnPassantMove),
    Promotion(PromotionMove),
}

#[derive(Debug)]
pub enum PromotionMove {
    PromotionCapture(PromotionCaptureMove),
    Queen(QueenPromotionMove),
    Rook(RookPromotionMove),
    Knight(KnightPromotionMove),
    Bishop(BishopPromotionMove),
}

#[derive(Debug)]
pub enum PromotionCaptureMove {
    Queen(QueenPromotionCaptureMove),
    Rook(RookPromotionCaptureMove),
    Knight(KnightPromotionCaptureMove),
    Bishop(BishopPromotionCaptureMove),
}

#[derive(Debug)]
pub struct QueenPromotionMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct RookPromotionMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct KnightPromotionMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct BishopPromotionMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct QueenPromotionCaptureMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct RookPromotionCaptureMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct KnightPromotionCaptureMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct BishopPromotionCaptureMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct EnPassantMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct PawnCaptureMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub enum PushMove {
    Single(SinglePushMove),
    Double(DoublePushMove),
}

#[derive(Debug)]
pub struct SinglePushMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct DoublePushMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub enum CastleMove {
    King(KingCastleMove),
    Queen(QueenCastleMove),
}

#[derive(Debug)]
pub struct KingCastleMove {
    coordinates: Coordinates,
}

#[derive(Debug)]
pub struct QueenCastleMove {
    coordinates: Coordinates,
}
