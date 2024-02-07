pub mod black;
pub mod white;

use black::BlackKnight;
use white::WhiteKnight;

use super::PieceExt;

#[derive(Debug)]
pub enum Knight {
    Black(BlackKnight),
    White(WhiteKnight),
}

pub trait KnightExt: PieceExt {}

impl KnightExt for Knight {}
impl PieceExt for Knight {}
