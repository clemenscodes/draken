pub mod black;
pub mod white;

use black::BlackQueen;
use white::WhiteQueen;

use super::PieceExt;

#[derive(Debug)]
pub enum Queen {
    Black(BlackQueen),
    White(WhiteQueen),
}

pub trait QueenExt: PieceExt {}

impl QueenExt for Queen {}
impl PieceExt for Queen {}
