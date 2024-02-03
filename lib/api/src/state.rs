#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Start,
    Playing,
    Checkmate,
    Stalemate,
    Resignation,
    Draw,
    DrawOffer,
    Promotion,
}
