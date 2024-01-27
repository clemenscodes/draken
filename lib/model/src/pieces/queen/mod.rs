mod black;
mod white;

use black::BlackQueen;
use white::WhiteQueen;

#[derive(Debug)]
pub enum Queen {
    Black(BlackQueen),
    White(WhiteQueen),
}
