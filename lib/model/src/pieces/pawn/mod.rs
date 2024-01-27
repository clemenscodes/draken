mod black;
mod white;

use black::BlackPawn;
use white::WhitePawn;

#[derive(Debug)]
pub enum Pawn {
    Black(BlackPawn),
    White(WhitePawn),
}
