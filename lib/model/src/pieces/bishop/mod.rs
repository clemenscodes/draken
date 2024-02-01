pub mod black;
pub mod white;

use black::BlackBishop;
use white::WhiteBishop;

use super::PieceExt;

#[derive(Debug)]
pub enum Bishop {
    Black(BlackBishop),
    White(WhiteBishop),
}

pub trait BishopExt: PieceExt {}

impl BishopExt for Bishop {}
impl PieceExt for Bishop {}
