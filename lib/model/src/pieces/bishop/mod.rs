mod black;
mod white;

use black::BlackBishop;
use white::WhiteBishop;

#[derive(Debug)]
pub enum Bishop {
    Black(BlackBishop),
    White(WhiteBishop),
}
