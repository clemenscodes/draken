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
    fn get_fen() -> String {
        todo!()
    }

    fn get_legal_moves(_square: api::Square) -> Vec<(Square, Square)> {
        todo!()
    }

    fn clear_error() {
        todo!()
    }
}

impl GameExt for ChessModel {
    fn init_game() {
        todo!()
    }

    fn start_game() {
        todo!()
    }

    fn resign() {
        todo!()
    }

    fn offer_draw() {
        todo!()
    }

    fn accept_draw() {
        todo!()
    }

    fn decline_draw() {
        todo!()
    }

    fn claim_draw() {
        todo!()
    }

    fn get_game_state() {
        todo!()
    }

    fn get_moves() {
        todo!()
    }

    fn make_move() {
        todo!()
    }

    fn promote_queen() {
        todo!()
    }

    fn promote_rook() {
        todo!()
    }

    fn promote_knight() {
        todo!()
    }

    fn promote_bishop() {
        todo!()
    }

    fn is_own_piece_on_square(_square: api::Square) -> bool {
        todo!()
    }
}

impl ForsythEdwardsNotationExt for ChessModel {
    fn get_piece_placement_data() -> Vec<String> {
        todo!()
    }

    fn is_white() -> bool {
        todo!()
    }

    fn get_castling() -> String {
        todo!()
    }

    fn get_white_king_castle() -> bool {
        todo!()
    }

    fn get_white_queen_castle() -> bool {
        todo!()
    }

    fn get_black_king_castle() -> bool {
        todo!()
    }

    fn get_black_queen_castle() -> bool {
        todo!()
    }

    fn get_en_passant() -> String {
        todo!()
    }

    fn get_half_move_clock() -> u8 {
        todo!()
    }

    fn get_full_move_clock() -> u16 {
        todo!()
    }
}
