pub mod black;
pub mod white;

use black::BlackKing;
use white::WhiteKing;

#[derive(Debug)]
pub enum King {
    Black(BlackKing),
    White(WhiteKing),
}
