pub mod black;
pub mod white;

use black::BlackRook;
use white::WhiteRook;

#[derive(Debug)]
pub enum Rook {
    Black(BlackRook),
    White(WhiteRook),
}
