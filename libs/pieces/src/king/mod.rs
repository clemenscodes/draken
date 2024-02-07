pub mod black;
pub mod white;

use black::BlackKing;
use white::WhiteKing;

use super::PieceExt;

#[derive(Debug)]
pub enum King {
    Black(BlackKing),
    White(WhiteKing),
}

pub trait KingExt: PieceExt {}

impl KingExt for King {}
impl PieceExt for King {}
