mod king;
mod queen;

use king::KingCastleMove;
use queen::QueenCastleMove;

#[derive(Debug)]
pub enum CastleMove {
    King(KingCastleMove),
    Queen(QueenCastleMove),
}
