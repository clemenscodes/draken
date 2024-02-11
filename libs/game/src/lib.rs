use api::{GameExt, MoveListExt, Square, State};
use bitboard::{Bitboard, BitboardExt};
use board::moves::{
    encoded_move::{EncodedMove, EncodedMoveExt},
    irreversible::{
        capture::CaptureMove,
        castle::{king::KingCastleMove, queen::QueenCastleMove},
        pawn::{
            enpassant::EnPassantMove,
            promotion::{
                bishop::BishopPromotionMove,
                capture::{
                    bishop::BishopPromotionCaptureMove, knight::KnightPromotionCaptureMove, queen::QueenPromotionCaptureMove,
                    rook::RookPromotionCaptureMove,
                },
                knight::KnightPromotionMove,
                queen::QueenPromotionMove,
                rook::RookPromotionMove,
            },
            push::DoublePushMove,
        },
    },
    list::MoveList,
    reversible::quiet::QuietMove,
    *,
};
use board::{Board, Verify};
use std::fmt::{Debug, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Game {
    board: Board,
    move_list: MoveList,
    state: State,
}

impl Game {
    pub fn new(board: Board, move_list: MoveList, state: State) -> Self {
        Self { board, move_list, state }
    }

    pub fn board(&self) -> Board {
        self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn move_list(&self) -> &MoveList {
        &self.move_list
    }

    pub fn move_list_mut(&mut self) -> &mut MoveList {
        &mut self.move_list
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    fn calculate_move(&mut self, source: Square, destination: Square) -> Result<u16, ()> {
        let board = self.board();
        let piece_index = board.get_piece_index(source)?;
        let pieces = board.pieces();
        match piece_index {
            0 => pieces.black_pieces().rook().verify(source, destination, board),
            1 => pieces.black_pieces().knight().verify(source, destination, board),
            2 => pieces.black_pieces().bishop().verify(source, destination, board),
            3 => pieces.black_pieces().queen().verify(source, destination, board),
            4 => pieces.black_pieces().king().verify(source, destination, board),
            5 => pieces.black_pieces().pawn().verify(source, destination, board),
            6 => pieces.white_pieces().rook().verify(source, destination, board),
            7 => pieces.white_pieces().knight().verify(source, destination, board),
            8 => pieces.white_pieces().bishop().verify(source, destination, board),
            9 => pieces.white_pieces().queen().verify(source, destination, board),
            10 => pieces.white_pieces().king().verify(source, destination, board),
            11 => pieces.white_pieces().pawn().verify(source, destination, board),
            _ => Err(()),
        }
    }

    fn piece_on_source(&self, source: Bitboard) -> bool {
        Bitboard::overlap(source, self.board().into())
    }

    fn perform_move(&mut self, encoded_move: EncodedMove) -> Result<(), ()> {
        let source = encoded_move.source();
        let destination = encoded_move.destination();
        let board = self.board_mut();
        match encoded_move.kind() {
            QUIET_MOVE => QuietMove::new(source, destination).march(board)?,
            DOUBLE_PAWN_PUSH => DoublePushMove::new(source, destination).march(board)?,
            KING_CASTLE => KingCastleMove::new(source, destination).march(board)?,
            QUEEN_CASTLE => QueenCastleMove::new(source, destination).march(board)?,
            CAPTURE => CaptureMove::new(source, destination).march(board)?,
            ENPASSANT => EnPassantMove::new(source, destination).march(board)?,
            KNIGHT_PROMOTION => KnightPromotionMove::new(source, destination).march(board)?,
            BISHOP_PROMOTION => BishopPromotionMove::new(source, destination).march(board)?,
            ROOK_PROMOTION => RookPromotionMove::new(source, destination).march(board)?,
            QUEEN_PROMOTION => QueenPromotionMove::new(source, destination).march(board)?,
            KNIGHT_PROMOTION_CAPTURE => KnightPromotionCaptureMove::new(source, destination).march(board)?,
            BISHOP_PROMOTION_CAPTURE => BishopPromotionCaptureMove::new(source, destination).march(board)?,
            ROOK_PROMOTION_CAPTURE => RookPromotionCaptureMove::new(source, destination).march(board)?,
            QUEEN_PROMOTION_CAPTURE => QueenPromotionCaptureMove::new(source, destination).march(board)?,
            _ => return Err(()),
        };
        self.move_list_mut().add(encoded_move.data());
        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game!")?;
        writeln!(f, "{}", self.board())?;
        writeln!(f, "{}", self.board().fen())?;
        writeln!(f, "{}", self.move_list())?;
        writeln!(f, "{}", self.state())
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl GameExt for Game {
    fn init_game(&mut self) {
        todo!()
    }

    fn start_game(&mut self) {
        todo!()
    }

    fn resign(&mut self) {
        todo!()
    }

    fn offer_draw(&self) {
        todo!()
    }

    fn accept_draw(&mut self) {
        todo!()
    }

    fn decline_draw(&self) {
        todo!()
    }

    fn claim_draw(&mut self) {
        todo!()
    }

    fn get_game_state(&self) {
        todo!()
    }

    fn get_moves(&self) {
        todo!()
    }

    fn promote_queen(&mut self) {
        todo!()
    }

    fn promote_rook(&mut self) {
        todo!()
    }

    fn promote_knight(&mut self) {
        todo!()
    }

    fn promote_bishop(&mut self) {
        todo!()
    }

    fn is_own_piece_on_square(&self, _square: Square) -> bool {
        todo!()
    }

    fn make_move(&mut self, source: Square, destination: Square) -> Result<(), ()> {
        let bitsource = Bitboard::from(source);
        if !self.move_list().validate(source, destination) {
            eprintln!("Source square can not be equal to destination square");
            return Err(());
        }
        if !self.piece_on_source(bitsource) {
            eprintln!("Source square can not be unoccupied");
            return Err(());
        }
        let data = self.calculate_move(source, destination)?;
        self.perform_move(EncodedMove::new(data))
    }

    fn ply(&self) -> u16 {
        self.move_list().ply()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::Square::*;

    #[test]
    fn test_game() {
        let game = Game::default();
        println!("{game}");
    }

    #[test]
    fn test_make_move() {
        let mut game = Game::default();
        assert_eq!(game.ply(), 0);
        game.make_move(E2, E7).unwrap();
        assert_eq!(game.ply(), 1);
        println!("{game}");
    }
}
