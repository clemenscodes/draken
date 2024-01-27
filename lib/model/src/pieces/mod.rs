mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

#[derive(Debug)]
pub enum Pieces {
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
    Pawn(Pawn),
}
