#![feature(lazy_cell)]
mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use bishop::{black::*, white::*, Bishop};
use bitboard::{Bitboard, BitboardExt};
use king::{black::*, white::*, King};
use knight::{black::*, white::*, Knight};
use pawn::{black::*, white::*, Pawn};
use queen::{black::*, white::*, Queen};
use rook::{black::*, white::*, Rook};
use std::{collections::HashMap, sync::LazyLock};

pub const NUM_PIECES: usize = 12;

pub const EMPTY_SYMBOL: char = ' ';

pub const PIECE_SYMBOLS: [char; NUM_PIECES] = [
    BlackRook::symbol(),
    BlackKnight::symbol(),
    BlackBishop::symbol(),
    BlackQueen::symbol(),
    BlackKing::symbol(),
    BlackPawn::symbol(),
    WhiteRook::symbol(),
    WhiteKnight::symbol(),
    WhiteBishop::symbol(),
    WhiteQueen::symbol(),
    WhiteKing::symbol(),
    WhitePawn::symbol(),
];

pub const UTF_SYMBOLS: [char; NUM_PIECES] = [
    BlackRook::utf_symbol(),
    BlackKnight::utf_symbol(),
    BlackBishop::utf_symbol(),
    BlackQueen::utf_symbol(),
    BlackKing::utf_symbol(),
    BlackPawn::utf_symbol(),
    WhiteRook::utf_symbol(),
    WhiteKnight::utf_symbol(),
    WhiteBishop::utf_symbol(),
    WhiteQueen::utf_symbol(),
    WhiteKing::utf_symbol(),
    WhitePawn::utf_symbol(),
];

pub const PIECE_BYTES: [u8; NUM_PIECES] = [
    BlackRook::symbol() as u8,
    BlackKnight::symbol() as u8,
    BlackBishop::symbol() as u8,
    BlackQueen::symbol() as u8,
    BlackKing::symbol() as u8,
    BlackPawn::symbol() as u8,
    WhiteRook::symbol() as u8,
    WhiteKnight::symbol() as u8,
    WhiteBishop::symbol() as u8,
    WhiteQueen::symbol() as u8,
    WhiteKing::symbol() as u8,
    WhitePawn::symbol() as u8,
];

pub const PIECE_INDEX_LOOKUP_MAP: LazyLock<HashMap<char, usize>> = LazyLock::new(|| {
    let mut piece_lookup: HashMap<char, usize> = HashMap::new();
    for (i, &piece) in PIECE_SYMBOLS.iter().enumerate() {
        piece_lookup.insert(piece, i);
    }
    piece_lookup
});

#[derive(Debug)]
pub enum Piece {
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
    Pawn(Pawn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceError {
    Invalid,
}

impl TryFrom<char> for Piece {
    type Error = PieceError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let piece: Piece = match value {
            piece if piece == PIECE_SYMBOLS[0] => Piece::Rook(Rook::Black(BlackRook::default())),
            piece if piece == PIECE_SYMBOLS[1] => Piece::Knight(Knight::Black(BlackKnight::default())),
            piece if piece == PIECE_SYMBOLS[2] => Piece::Bishop(Bishop::Black(BlackBishop::default())),
            piece if piece == PIECE_SYMBOLS[3] => Piece::Queen(Queen::Black(BlackQueen::default())),
            piece if piece == PIECE_SYMBOLS[4] => Piece::King(King::Black(BlackKing::default())),
            piece if piece == PIECE_SYMBOLS[5] => Piece::Pawn(Pawn::Black(BlackPawn::default())),
            piece if piece == PIECE_SYMBOLS[6] => Piece::Rook(Rook::White(WhiteRook::default())),
            piece if piece == PIECE_SYMBOLS[7] => Piece::Knight(Knight::White(WhiteKnight::default())),
            piece if piece == PIECE_SYMBOLS[8] => Piece::Bishop(Bishop::White(WhiteBishop::default())),
            piece if piece == PIECE_SYMBOLS[9] => Piece::Queen(Queen::White(WhiteQueen::default())),
            piece if piece == PIECE_SYMBOLS[10] => Piece::King(King::White(WhiteKing::default())),
            piece if piece == PIECE_SYMBOLS[11] => Piece::Pawn(Pawn::White(WhitePawn::default())),
            invalid => {
                eprintln!("Invalid character {invalid}");
                return Err(Self::Error::Invalid);
            }
        };
        Ok(piece)
    }
}

impl From<Pawn> for Piece {
    fn from(v: Pawn) -> Self {
        Self::Pawn(v)
    }
}

impl From<King> for Piece {
    fn from(v: King) -> Self {
        Self::King(v)
    }
}

impl From<Queen> for Piece {
    fn from(v: Queen) -> Self {
        Self::Queen(v)
    }
}

impl From<Bishop> for Piece {
    fn from(v: Bishop) -> Self {
        Self::Bishop(v)
    }
}

impl From<Knight> for Piece {
    fn from(v: Knight) -> Self {
        Self::Knight(v)
    }
}

impl From<Rook> for Piece {
    fn from(v: Rook) -> Self {
        Self::Rook(v)
    }
}

pub trait PieceExt {}

impl PieceExt for Piece {}

impl Piece {
    pub fn get_board(&mut self) -> &mut Bitboard {
        match self {
            Piece::Rook(rook) => match rook {
                Rook::Black(rook) => rook.bitboard_mut(),
                Rook::White(rook) => rook.bitboard_mut(),
            },
            Piece::Knight(knight) => match knight {
                Knight::Black(knight) => knight.bitboard_mut(),
                Knight::White(knight) => knight.bitboard_mut(),
            },
            Piece::Bishop(bishop) => match bishop {
                Bishop::Black(bishop) => bishop.bitboard_mut(),
                Bishop::White(bishop) => bishop.bitboard_mut(),
            },
            Piece::Queen(queen) => match queen {
                Queen::Black(queen) => queen.bitboard_mut(),
                Queen::White(queen) => queen.bitboard_mut(),
            },
            Piece::King(king) => match king {
                King::Black(king) => king.bitboard_mut(),
                King::White(king) => king.bitboard_mut(),
            },
            Piece::Pawn(pawn) => match pawn {
                Pawn::Black(pawn) => pawn.bitboard_mut(),
                Pawn::White(pawn) => pawn.bitboard_mut(),
            },
        }
    }

    pub fn set_on_square(&mut self, rank: u8, file: u8) {
        let board = Bitboard::try_from((rank as usize, file as usize)).unwrap();
        self.get_board().self_merge(board);
    }
}

#[derive(Default, Debug)]
pub struct Pieces {
    white_king: WhiteKing,
    black_king: BlackKing,
    white_bishop: WhiteBishop,
    black_bishop: BlackBishop,
    white_queen: WhiteQueen,
    black_queen: BlackQueen,
    white_rook: WhiteRook,
    black_rook: BlackRook,
    white_knight: WhiteKnight,
    black_knight: BlackKnight,
    white_pawn: WhitePawn,
    black_pawn: BlackPawn,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
    occupied_squares: Bitboard,
    empty_squares: Bitboard,
}

impl Pieces {
    pub fn white_king(&self) -> &WhiteKing {
        &self.white_king
    }

    pub fn black_king(&self) -> &BlackKing {
        &self.black_king
    }

    pub fn white_bishop(&self) -> &WhiteBishop {
        &self.white_bishop
    }

    pub fn black_bishop(&self) -> &BlackBishop {
        &self.black_bishop
    }

    pub fn white_queen(&self) -> &WhiteQueen {
        &self.white_queen
    }

    pub fn black_queen(&self) -> &BlackQueen {
        &self.black_queen
    }

    pub fn white_rook(&self) -> &WhiteRook {
        &self.white_rook
    }

    pub fn black_rook(&self) -> &BlackRook {
        &self.black_rook
    }

    pub fn white_knight(&self) -> &WhiteKnight {
        &self.white_knight
    }

    pub fn black_knight(&self) -> &BlackKnight {
        &self.black_knight
    }

    pub fn white_pawn(&self) -> &WhitePawn {
        &self.white_pawn
    }

    pub fn black_pawn(&self) -> &BlackPawn {
        &self.black_pawn
    }

    pub fn white_king_mut(&mut self) -> &mut WhiteKing {
        &mut self.white_king
    }

    pub fn black_king_mut(&mut self) -> &mut BlackKing {
        &mut self.black_king
    }

    pub fn white_bishop_mut(&mut self) -> &mut WhiteBishop {
        &mut self.white_bishop
    }

    pub fn black_bishop_mut(&mut self) -> &mut BlackBishop {
        &mut self.black_bishop
    }

    pub fn white_queen_mut(&mut self) -> &mut WhiteQueen {
        &mut self.white_queen
    }

    pub fn black_queen_mut(&mut self) -> &mut BlackQueen {
        &mut self.black_queen
    }

    pub fn white_rook_mut(&mut self) -> &mut WhiteRook {
        &mut self.white_rook
    }

    pub fn black_rook_mut(&mut self) -> &mut BlackRook {
        &mut self.black_rook
    }

    pub fn white_knight_mut(&mut self) -> &mut WhiteKnight {
        &mut self.white_knight
    }

    pub fn black_knight_mut(&mut self) -> &mut BlackKnight {
        &mut self.black_knight
    }

    pub fn white_pawn_mut(&mut self) -> &mut WhitePawn {
        &mut self.white_pawn
    }

    pub fn black_pawn_mut(&mut self) -> &mut BlackPawn {
        &mut self.black_pawn
    }

    pub fn create(symbol: char, rank: u8, file: u8) -> Piece {
        let mut piece: Piece = Piece::try_from(symbol).unwrap();
        piece.set_on_square(rank, file);
        piece
    }

    pub fn get_all_pieces(&self) -> [Bitboard; NUM_PIECES] {
        [
            self.black_rook().bitboard(),
            self.black_knight().bitboard(),
            self.black_bishop().bitboard(),
            self.black_queen().bitboard(),
            self.black_king().bitboard(),
            self.black_pawn().bitboard(),
            self.white_rook().bitboard(),
            self.white_knight().bitboard(),
            self.white_bishop().bitboard(),
            self.white_queen().bitboard(),
            self.white_king().bitboard(),
            self.white_pawn().bitboard(),
        ]
    }

    pub fn get_piece_symbol(&self, bitboard: Bitboard) -> char {
        let all_pieces = self.get_all_pieces();
        for (index, piece) in all_pieces.iter().enumerate() {
            if bitboard.self_overlap(*piece) {
                return PIECE_SYMBOLS[index];
            }
        }
        EMPTY_SYMBOL
    }

    pub fn get_utf_piece_symbol(&self, bitboard: Bitboard) -> char {
        let all_pieces = self.get_all_pieces();
        for (index, piece) in all_pieces.iter().enumerate() {
            if bitboard.self_overlap(*piece) {
                return UTF_SYMBOLS[index];
            }
        }
        EMPTY_SYMBOL
    }

    pub fn merge_piece(&mut self, mut piece: Piece) {
        let board = piece.get_board().clone();
        match piece {
            Piece::Rook(rook) => match rook {
                Rook::Black(_) => self.black_rook_mut().bitboard_mut().self_merge(board),
                Rook::White(_) => self.white_rook_mut().bitboard_mut().self_merge(board),
            },
            Piece::Knight(knight) => match knight {
                Knight::Black(_) => self.black_knight_mut().bitboard_mut().self_merge(board),
                Knight::White(_) => self.white_knight_mut().bitboard_mut().self_merge(board),
            },
            Piece::Bishop(bishop) => match bishop {
                Bishop::Black(_) => self.black_bishop_mut().bitboard_mut().self_merge(board),
                Bishop::White(_) => self.white_bishop_mut().bitboard_mut().self_merge(board),
            },
            Piece::Queen(queen) => match queen {
                Queen::Black(_) => self.black_queen_mut().bitboard_mut().self_merge(board),
                Queen::White(_) => self.white_queen_mut().bitboard_mut().self_merge(board),
            },
            Piece::King(king) => match king {
                King::Black(_) => self.black_king_mut().bitboard_mut().self_merge(board),
                King::White(_) => self.white_king_mut().bitboard_mut().self_merge(board),
            },
            Piece::Pawn(pawn) => match pawn {
                Pawn::Black(_) => self.black_pawn_mut().bitboard_mut().self_merge(board),
                Pawn::White(_) => self.white_pawn_mut().bitboard_mut().self_merge(board),
            },
        };
    }

    pub fn white_pieces(&self) -> Bitboard {
        self.white_pieces
    }

    pub fn white_pieces_mut(&mut self) -> &mut Bitboard {
        &mut self.white_pieces
    }

    pub fn black_pieces(&self) -> Bitboard {
        self.black_pieces
    }

    pub fn black_pieces_mut(&mut self) -> &mut Bitboard {
        &mut self.black_pieces
    }

    pub fn occupied_squares(&self) -> Bitboard {
        self.occupied_squares
    }

    pub fn occupied_squares_mut(&mut self) -> &mut Bitboard {
        &mut self.occupied_squares
    }

    pub fn empty_squares(&self) -> Bitboard {
        self.empty_squares
    }

    pub fn empty_squares_mut(&mut self) -> &mut Bitboard {
        &mut self.empty_squares
    }
}

impl From<[[u8; 8]; 8]> for Pieces {
    fn from(value: [[u8; 8]; 8]) -> Self {
        let mut reverse_rank = 0u8;
        let mut pieces = Self::default();
        for rank in (0..8u8).rev() {
            let mut file = 0u8;
            for piece in value[reverse_rank as usize] {
                if piece == 0 {
                    file += 1;
                    continue;
                }
                let piece = Self::create(piece as char, rank, file);
                pieces.merge_piece(piece);
                file += 1;
            }
            reverse_rank += 1;
        }
        pieces
    }
}

pub trait PiecesExt {}

impl PiecesExt for Pieces {}
