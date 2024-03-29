pub mod black;
pub mod white;

use super::PieceExt;
use crate::{Board, Verify};
use api::Square;
use bitboard::Bitboard;
use black::BlackKing;
use std::error::Error;
use white::WhiteKing;

#[derive(Debug)]
pub enum King {
    Black(BlackKing),
    White(WhiteKing),
}

pub trait KingExt: PieceExt {}

impl From<WhiteKing> for King {
    fn from(v: WhiteKing) -> Self {
        Self::White(v)
    }
}

impl From<BlackKing> for King {
    fn from(v: BlackKing) -> Self {
        Self::Black(v)
    }
}

impl PieceExt for King {
    fn get_attacks(&self, _piece: Bitboard, _board: Board) -> bitboard::Bitboard {
        todo!()
    }
}

impl Verify for King {
    fn verify(&self, _source: Square, _destination: Square, _promotion: Option<char>, _board: Board) -> Result<u16, Box<dyn Error>> {
        todo!()
    }
}

impl KingExt for King {}
