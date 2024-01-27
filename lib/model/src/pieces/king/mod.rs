mod black;
mod white;

use black::BlackKing;
use white::WhiteKing;

#[derive(Debug)]
pub enum King {
    Black(BlackKing),
    White(WhiteKing),
}
