use api::{ForsythEdwardsNotationExt, GameExt, Model, Square};
use game::Game;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessModel {
    game: Game,
}

impl ChessModel {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn game_mut(&mut self) -> &mut Game {
        &mut self.game
    }
}

impl Model for ChessModel {
    fn get_fen(&self) -> String {
        self.game().board().fen().to_string()
    }

    fn get_legal_moves(&self, _square: Square) -> Vec<(Square, Square)> {
        todo!()
    }

    fn clear_error(&mut self) {
        todo!()
    }
}

impl GameExt for ChessModel {
    fn init_game(&mut self) {
        self.game_mut().init_game()
    }

    fn start_game(&mut self) {
        self.game_mut().start_game()
    }

    fn resign(&mut self) {
        self.game_mut().resign()
    }

    fn offer_draw(&self) {
        self.game().offer_draw()
    }

    fn accept_draw(&mut self) {
        self.game_mut().accept_draw()
    }

    fn decline_draw(&self) {
        self.game().decline_draw()
    }

    fn claim_draw(&mut self) {
        self.game_mut().claim_draw()
    }

    fn get_game_state(&self) {
        self.game().get_game_state()
    }

    fn get_moves(&self) {
        self.game().get_moves()
    }

    fn make_move(&mut self) {
        self.game_mut().make_move()
    }

    fn promote_queen(&mut self) {
        self.game_mut().promote_queen()
    }

    fn promote_rook(&mut self) {
        self.game_mut().promote_rook()
    }

    fn promote_knight(&mut self) {
        self.game_mut().promote_knight()
    }

    fn promote_bishop(&mut self) {
        self.game_mut().promote_bishop()
    }

    fn is_own_piece_on_square(&self, square: Square) -> bool {
        self.game().is_own_piece_on_square(square)
    }
}

impl ForsythEdwardsNotationExt for ChessModel {
    fn get_piece_placement_data(&self) -> Vec<String> {
        self.game().board().fen().get_piece_placement_data()
    }

    fn is_white(&self) -> bool {
        self.game().board().fen().is_white()
    }

    fn get_castling(&self) -> String {
        self.game().board().fen().get_castling()
    }

    fn get_white_king_castle(&self) -> bool {
        self.game().board().fen().get_white_king_castle()
    }

    fn get_white_queen_castle(&self) -> bool {
        self.game().board().fen().get_white_queen_castle()
    }

    fn get_black_king_castle(&self) -> bool {
        self.game().board().fen().get_black_king_castle()
    }

    fn get_black_queen_castle(&self) -> bool {
        self.game().board().fen().get_black_queen_castle()
    }

    fn get_en_passant(&self) -> String {
        self.game().board().fen().get_en_passant()
    }

    fn get_half_move_clock(&self) -> u8 {
        self.game().board().fen().get_half_move_clock()
    }

    fn get_full_move_clock(&self) -> u16 {
        self.game().board().fen().get_full_move_clock()
    }
}
