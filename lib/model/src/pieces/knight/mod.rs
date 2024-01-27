pub mod black;
pub mod white;

use black::BlackKnight;
use white::WhiteKnight;

#[derive(Debug)]
pub enum Knight {
    Black(BlackKnight),
    White(WhiteKnight),
}
