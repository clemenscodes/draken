use std::fmt::{Debug, Display};

use api::{GameExt, MoveListExt, Square, State};
use bitboard::{Bitboard, BitboardExt};
use board::Board;
use moves::list::MoveList;
use pieces::March;

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

    fn perform_move(&mut self, source: Square, destination: Square) -> Result<u16, ()> {
        let piece_index = self.get_piece_index(source)?;
        let pieces = self.board_mut().pieces_mut();
        match piece_index {
            0 => pieces.black_pieces_mut().rook_mut().march(source, destination),
            1 => pieces.black_pieces_mut().knight_mut().march(source, destination),
            2 => pieces.black_pieces_mut().bishop_mut().march(source, destination),
            3 => pieces.black_pieces_mut().queen_mut().march(source, destination),
            4 => pieces.black_pieces_mut().king_mut().march(source, destination),
            5 => pieces.black_pieces_mut().pawn().march(source, destination),
            6 => pieces.white_pieces_mut().rook_mut().march(source, destination),
            7 => pieces.white_pieces_mut().knight_mut().march(source, destination),
            8 => pieces.white_pieces_mut().bishop_mut().march(source, destination),
            9 => pieces.white_pieces_mut().queen_mut().march(source, destination),
            10 => pieces.white_pieces_mut().king_mut().march(source, destination),
            11 => pieces.white_pieces_mut().pawn_mut().march(source, destination),
            _ => Err(()),
        }
    }

    fn get_piece_index(&self, source: Square) -> Result<usize, ()> {
        let pieces = self.board().pieces().get_all_pieces();
        for i in 0..pieces.len() {
            if Bitboard::overlap(Bitboard::from(source), pieces[i]) {
                return Ok(i);
            }
        }
        Err(())
    }

    fn piece_on_source(&self, source: Bitboard) -> bool {
        Bitboard::overlap(source, self.board().into())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game!")?;
        writeln!(f, "{}", self.board())?;
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
        let encoded_move: u16 = self.perform_move(source, destination)?;
        self.move_list_mut().add(encoded_move)
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
        game.make_move(E2, E4).unwrap();
        assert_eq!(game.ply(), 1);
        println!("{game}");
    }
}
