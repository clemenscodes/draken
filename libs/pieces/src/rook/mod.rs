pub mod black;
pub mod white;

use black::BlackRook;
use white::WhiteRook;

use super::PieceExt;

#[derive(Debug)]
pub enum Rook {
    Black(BlackRook),
    White(WhiteRook),
}

pub trait RookExt: PieceExt {}

impl RookExt for Rook {}
impl PieceExt for Rook {}
