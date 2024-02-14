mod bishop;
mod black_pieces;
mod king;
mod knight;
mod pawn;
pub mod piece;
mod queen;
mod rook;
mod white_pieces;

use self::{piece::*, white_pieces::WhitePieces};
pub use bishop::{black::*, white::*, Bishop};
pub use bitboard::{Bitboard, BitboardExt};
use black_pieces::BlackPieces;
pub use king::{black::*, white::*, King};
pub use knight::{black::*, white::*, Knight};
pub use pawn::{black::*, white::*, Pawn, PawnExt};
pub use queen::{black::*, white::*, Queen};
pub use rook::{black::*, white::*, Rook};
pub use std::{collections::HashMap, fmt::Debug, sync::LazyLock, vec};

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

pub static PIECE_INDEX_LOOKUP_MAP: LazyLock<HashMap<char, usize>> = LazyLock::new(|| {
    let mut piece_lookup: HashMap<char, usize> = HashMap::new();
    for (i, &piece) in PIECE_SYMBOLS.iter().enumerate() {
        piece_lookup.insert(piece, i);
    }
    piece_lookup
});

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pieces {
    white_pieces: WhitePieces,
    black_pieces: BlackPieces,
    occupied_squares: Bitboard,
    empty_squares: Bitboard,
}

pub trait PiecesExt {}

impl Pieces {
    pub fn white_pieces(&self) -> WhitePieces {
        self.white_pieces
    }

    pub fn white_pieces_mut(&mut self) -> &mut WhitePieces {
        &mut self.white_pieces
    }

    pub fn black_pieces(&self) -> BlackPieces {
        self.black_pieces
    }

    pub fn black_pieces_mut(&mut self) -> &mut BlackPieces {
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

    pub fn get_all_pieces_mut(&mut self) -> [Bitboard; NUM_PIECES] {
        [
            self.black_pieces_mut().rook_mut().bitboard(),
            self.black_pieces_mut().knight_mut().bitboard(),
            self.black_pieces_mut().bishop_mut().bitboard(),
            self.black_pieces_mut().queen_mut().bitboard(),
            self.black_pieces_mut().king_mut().bitboard(),
            self.black_pieces_mut().pawn_mut().bitboard(),
            self.white_pieces_mut().rook_mut().bitboard(),
            self.white_pieces_mut().knight_mut().bitboard(),
            self.white_pieces_mut().bishop_mut().bitboard(),
            self.white_pieces_mut().queen_mut().bitboard(),
            self.white_pieces_mut().king_mut().bitboard(),
            self.white_pieces_mut().pawn_mut().bitboard(),
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
        let board = *piece.get_board();
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

    fn set_occupied_squares(&mut self, occupied_squares: Bitboard) {
        self.occupied_squares = occupied_squares;
    }

    fn set_empty_squares(&mut self, empty_squares: Bitboard) {
        self.empty_squares = empty_squares;
    }

    fn update_occupied_squares(&mut self) {
        self.set_occupied_squares(Bitboard::merge_many(self.get_all_pieces().to_vec()));
    }

    fn update_empty_squares(&mut self) {
        let occupied_squares = self.occupied_squares();
        self.set_empty_squares(!occupied_squares);
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

impl From<Pieces> for Bitboard {
    fn from(val: Pieces) -> Self {
        Bitboard::merge_many(vec![val.white_pieces().into(), val.black_pieces().into()])
    }
}

impl PiecesExt for Pieces {}
