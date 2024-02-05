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
use std::{collections::HashMap, fmt::Debug, sync::LazyLock, vec};

pub const NUM_COLOR_PIECES: usize = 6;
pub const NUM_COLORS: usize = 2;
pub const NUM_PIECES: usize = NUM_COLORS * NUM_COLOR_PIECES;

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
            _ => return Err(Self::Error::Invalid),
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
    fn get_board(&mut self) -> &mut Bitboard {
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

    fn set_on_square(&mut self, rank: u8, file: u8) {
        let board = Bitboard::try_from((rank as usize, file as usize)).unwrap();
        self.get_board().self_merge(board);
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct WhitePieces {
    rook: WhiteRook,
    knight: WhiteKnight,
    bishop: WhiteBishop,
    queen: WhiteQueen,
    king: WhiteKing,
    pawn: WhitePawn,
}

impl WhitePieces {
    fn rook(&self) -> WhiteRook {
        self.rook
    }

    fn rook_mut(&mut self) -> &mut WhiteRook {
        &mut self.rook
    }

    fn knight(&self) -> WhiteKnight {
        self.knight
    }

    fn knight_mut(&mut self) -> &mut WhiteKnight {
        &mut self.knight
    }

    fn bishop(&self) -> WhiteBishop {
        self.bishop
    }

    fn bishop_mut(&mut self) -> &mut WhiteBishop {
        &mut self.bishop
    }

    fn queen(&self) -> WhiteQueen {
        self.queen
    }

    fn queen_mut(&mut self) -> &mut WhiteQueen {
        &mut self.queen
    }

    fn king(&self) -> WhiteKing {
        self.king
    }

    fn king_mut(&mut self) -> &mut WhiteKing {
        &mut self.king
    }

    fn pawn(&self) -> WhitePawn {
        self.pawn
    }

    fn pawn_mut(&mut self) -> &mut WhitePawn {
        &mut self.pawn
    }
}

impl Into<Bitboard> for WhitePieces {
    fn into(self) -> Bitboard {
        Bitboard::merge_many(vec![
            self.rook.bitboard(),
            self.knight.bitboard(),
            self.bishop.bitboard(),
            self.queen.bitboard(),
            self.king.bitboard(),
            self.pawn.bitboard(),
        ])
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct BlackPieces {
    rook: BlackRook,
    knight: BlackKnight,
    bishop: BlackBishop,
    queen: BlackQueen,
    king: BlackKing,
    pawn: BlackPawn,
}

impl BlackPieces {
    fn rook(&self) -> BlackRook {
        self.rook
    }

    fn rook_mut(&mut self) -> &mut BlackRook {
        &mut self.rook
    }

    fn knight(&self) -> BlackKnight {
        self.knight
    }

    fn knight_mut(&mut self) -> &mut BlackKnight {
        &mut self.knight
    }

    fn bishop(&self) -> BlackBishop {
        self.bishop
    }

    fn bishop_mut(&mut self) -> &mut BlackBishop {
        &mut self.bishop
    }

    fn queen(&self) -> BlackQueen {
        self.queen
    }

    fn queen_mut(&mut self) -> &mut BlackQueen {
        &mut self.queen
    }

    fn king(&self) -> BlackKing {
        self.king
    }

    fn king_mut(&mut self) -> &mut BlackKing {
        &mut self.king
    }

    fn pawn(&self) -> BlackPawn {
        self.pawn
    }

    fn pawn_mut(&mut self) -> &mut BlackPawn {
        &mut self.pawn
    }
}

impl Into<Bitboard> for BlackPieces {
    fn into(self) -> Bitboard {
        Bitboard::merge_many(vec![
            self.rook.bitboard(),
            self.knight.bitboard(),
            self.bishop.bitboard(),
            self.queen.bitboard(),
            self.king.bitboard(),
            self.pawn.bitboard(),
        ])
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pieces {
    white_pieces: WhitePieces,
    black_pieces: BlackPieces,
    occupied_squares: Bitboard,
    empty_squares: Bitboard,
}

impl Pieces {
    fn white_pieces(&self) -> WhitePieces {
        self.white_pieces
    }

    fn white_pieces_mut(&mut self) -> &mut WhitePieces {
        &mut self.white_pieces
    }

    fn black_pieces(&self) -> BlackPieces {
        self.black_pieces
    }

    fn black_pieces_mut(&mut self) -> &mut BlackPieces {
        &mut self.black_pieces
    }

    pub fn occupied_squares(&self) -> Bitboard {
        self.occupied_squares
    }

    pub fn empty_squares(&self) -> Bitboard {
        self.empty_squares
    }

    pub fn get_all_pieces(&self) -> [Bitboard; NUM_PIECES] {
        [
            self.black_pieces().rook().bitboard(),
            self.black_pieces().knight().bitboard(),
            self.black_pieces().bishop().bitboard(),
            self.black_pieces().queen().bitboard(),
            self.black_pieces().king().bitboard(),
            self.black_pieces().pawn().bitboard(),
            self.white_pieces().rook().bitboard(),
            self.white_pieces().knight().bitboard(),
            self.white_pieces().bishop().bitboard(),
            self.white_pieces().queen().bitboard(),
            self.white_pieces().king().bitboard(),
            self.white_pieces().pawn().bitboard(),
        ]
    }

    pub fn get_piece_symbol(&self, bitboard: Bitboard, symbols: [char; NUM_PIECES]) -> char {
        let all_pieces = self.get_all_pieces();
        for (index, piece) in all_pieces.iter().enumerate() {
            if bitboard.self_overlap(*piece) {
                return symbols[index];
            }
        }
        EMPTY_SYMBOL
    }

    fn create(symbol: char, rank: u8, file: u8) -> Piece {
        let mut piece: Piece = Piece::try_from(symbol).unwrap();
        piece.set_on_square(rank, file);
        piece
    }

    fn merge_piece(&mut self, mut piece: Piece) {
        let board = piece.get_board().clone();
        match piece {
            Piece::Rook(rook) => match rook {
                Rook::Black(_) => self.black_pieces_mut().rook_mut().bitboard_mut().self_merge(board),
                Rook::White(_) => self.white_pieces_mut().rook_mut().bitboard_mut().self_merge(board),
            },
            Piece::Knight(knight) => match knight {
                Knight::Black(_) => self.black_pieces_mut().knight_mut().bitboard_mut().self_merge(board),
                Knight::White(_) => self.white_pieces_mut().knight_mut().bitboard_mut().self_merge(board),
            },
            Piece::Bishop(bishop) => match bishop {
                Bishop::Black(_) => self.black_pieces_mut().bishop_mut().bitboard_mut().self_merge(board),
                Bishop::White(_) => self.white_pieces_mut().bishop_mut().bitboard_mut().self_merge(board),
            },
            Piece::Queen(queen) => match queen {
                Queen::Black(_) => self.black_pieces_mut().queen_mut().bitboard_mut().self_merge(board),
                Queen::White(_) => self.white_pieces_mut().queen_mut().bitboard_mut().self_merge(board),
            },
            Piece::King(king) => match king {
                King::Black(_) => self.black_pieces_mut().king_mut().bitboard_mut().self_merge(board),
                King::White(_) => self.white_pieces_mut().king_mut().bitboard_mut().self_merge(board),
            },
            Piece::Pawn(pawn) => match pawn {
                Pawn::Black(_) => self.black_pieces_mut().pawn_mut().bitboard_mut().self_merge(board),
                Pawn::White(_) => self.white_pieces_mut().pawn_mut().bitboard_mut().self_merge(board),
            },
        };
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
        let occupied_squares = Bitboard::merge_many(pieces.get_all_pieces().to_vec());
        let empty_squares = !occupied_squares;
        Self {
            black_pieces: pieces.black_pieces(),
            white_pieces: pieces.white_pieces(),
            occupied_squares,
            empty_squares,
        }
    }
}

pub trait PiecesExt {}

impl PiecesExt for Pieces {}