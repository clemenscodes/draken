use board::Board;
use moves::list::MoveList;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Game {
    board: Board,
    move_list: MoveList,
}

impl Game {
    pub fn new(board: Board, move_list: MoveList) -> Self {
        Self { board, move_list }
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
}
