use std::fmt::{Debug, Display};

use api::{GameExt, State};
use board::Board;
use moves::list::MoveList;

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

    fn is_own_piece_on_square(&self, _square: api::Square) -> bool {
        todo!()
    }

    fn make_move(&mut self, _source: api::Square, _destination: api::Square) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let game = Game::default();
        println!("{game}");
    }
}
